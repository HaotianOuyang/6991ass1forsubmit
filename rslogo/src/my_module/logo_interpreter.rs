use crate::my_module::logo_parser::{Program, Statement, Expr, ProgramProcedure};


use unsvg::{Image, COLORS, get_end_coordinates};


pub struct Interpreter {
    x: i32,
    y: i32,
    heading: i32,
    pen_down: bool,
    pen_color: u32,
    variables: std::collections::HashMap<String, String>,
    image: Image,
}


impl Interpreter {
    pub fn new(width: i32, height: i32) -> Self {
        Interpreter {
            x: width / 2 as i32,
            y: height / 2 as i32,
            heading: 0,
            pen_down: true,
            pen_color: 7, 
            variables: std::collections::HashMap::new(),
            image: Image::new(width as u32, height as u32),
        }
    }

    pub fn interpret(&mut self, program: &Program) -> Result<Image, String> {
        println!("Starting program interpretation");
        let statement_count = program.statements.len();
        println!("Starting program interpretation. Total statements: {}", statement_count);
        for (i, statement) in program.statements.iter().enumerate() {
            println!("Executing statement {}: {:?}", i, statement);
            self.execute_statement(statement)?;

        }
        println!("Program interpretation completed");
        Ok(self.image.clone())
    }

    fn execute_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::PenUp => {
                self.pen_down = false;
                Ok(())
            },
            Statement::PenDown => {
                self.pen_down = true;
                Ok(())
            },
            Statement::Forward(expr) => {
                println!("Executing FORWARD with expression: {:?}", expr);
                let distance = self.evaluate_expr(expr)?.parse::<i32>().map_err(|_| format!("Cannot convert to number"))?;
                println!("Forward distance evaluated to: {}", distance);
                self.move_forward(distance);
                Ok(())
            },
            Statement::Back(expr) => {
                let distance = self.evaluate_expr(expr)?.parse::<i32>().map_err(|_| format!("Cannot convert to number"))?;
                self.move_back(distance);
                Ok(())
            },
            Statement::Left(expr) => {
                let distance = self.evaluate_expr(expr)?.parse::<i32>().map_err(|_| format!("Cannot convert to number"))?;
                self.move_left(distance);
                Ok(())
            },
            Statement::Right(expr) => {
                let distance = self.evaluate_expr(expr)?.parse::<i32>().map_err(|_| format!("Cannot convert to number"))?;
                self.move_right(distance);
                Ok(())
            },
            Statement::Turn(expr) => {
                let angle = self.evaluate_expr(expr)?.parse::<i32>().map_err(|_| format!("Cannot convert to number"))?;
                self.turn(angle);
                Ok(())
            },
            Statement::SetPenColor(expr) =>{
                let color_code = self.evaluate_expr(expr)?.parse::<i32>().map_err(|_| format!("Cannot convert to number"))?;
                self.set_color(color_code as u32);
                Ok(())
            },
            Statement::MakeVariable(variable, expr) => {
                println!("Executing MAKE - Variable expr: {:?}, Value expr: {:?}", variable, expr);
                
                // 获取变量名
                let var_name = self.evaluate_expr(variable)?;
                println!("Variable name evaluated to: {}", var_name);
                
                // 获取值
                let value = self.evaluate_expr(expr)?;
                println!("Value evaluated to: {}", value);
                
                // 存储到 HashMap 中
                self.variables.insert(var_name, value);
                println!("Variable stored in HashMap");
                Ok(())
            },
            Statement::AddAssign(variable, expr )=>{
                let var_name = match variable {
                    Expr::Variable(ref name) => {
                        self.variables.get(name)
                            .ok_or_else(|| format!("Undefined variable: {}", name))?
                            .clone()
                    },
                    Expr::Literal(name) => name.clone(),
                    _ => return Err(format!("Invalid variable name expression: {:?}", variable)),
                };

                let current_value = self.variables.get(&var_name)
                        .ok_or_else(|| format!("Undefined variable: {}", var_name))?
                        .parse::<i32>()
                        .map_err(|_| format!("Cannot convert '{}' to number", var_name))?;

                let add_value = self.evaluate_expr(expr)?.parse::<i32>().map_err(|_| format!("Cannot convert '{}' to number", var_name))?;

                let new_value = (current_value + add_value).to_string();
    
                // 更新变量值
                self.variables.insert(var_name, new_value);
                Ok(())
            },
            Statement::SetHeading(expr)=>{
                let direction = self.evaluate_expr(expr)?.parse::<i32>().map_err(|_| format!("Cannot convert to number"))?.abs();
                self.set_heading(direction);
                Ok(())
            },
            Statement::SetX(expr) =>{
                let x = self.evaluate_expr(expr)?.parse::<i32>().map_err(|_| format!("Cannot convert to number"))?.abs();
                self.set_x(x);
                Ok(())
            },
            Statement::SetY(expr) =>{
                let y = self.evaluate_expr(expr)?.parse::<i32>().map_err(|_| format!("Cannot convert to number"))?.abs()    ;
                self.set_y(y);
                Ok(())
            },
            Statement::IF(condition, statements)=>{
                if self.evaluate_expr(condition)?.to_uppercase() == String::from("TRUE"){
                    for stmt in statements{
                        println!("Condition is true, executing statements: {:?}", statements);
                        self.execute_statement(stmt)?;
                    }
                }
                Ok(())
            },
            Statement::WHILE(condition, statements)=>{
                while self.evaluate_expr(condition)?.to_uppercase() == String::from("TRUE"){
                    for stmt in statements{
                        self.execute_statement(stmt)?;
                    }
                }
                Ok(())
            },
            Statement::Comment => Ok(()),
            _ => Err(format!("Unimplemented statement: {:?}", statement)),
        }
    }

    fn evaluate_expr(&self, expr: &Expr) -> Result<String, String> {
        println!("Evaluating expression: {:?}", expr); // 添加调试信息
        match expr {
            Expr::Literal(s) => {
                println!("Evaluating literal: {}", s);
                Ok(s.to_string())  // 直接返回字符串
            },
            Expr::Variable(name) => {
                println!("Evaluating variable: {}", name);
                // 获取变量值
                let value = self.variables.get(name)
                    .ok_or_else(|| format!("Undefined variable: {}", name))?;
                println!("Variable {} value is: {}", name, value);
                Ok(value.to_string())  // 直接返回字符串
            },
            Expr::ProgramProcedure(proc) => {
                println!("Evaluating program procedure: {:?}", proc);
                // 程序过程需要返回数字,但转换为字符串
                match proc {
                    ProgramProcedure::XCOR => Ok(self.x.to_string()),
                    ProgramProcedure::YCOR => Ok(self.y.to_string()),
                    ProgramProcedure::HEADING => Ok(self.heading.to_string()),
                    ProgramProcedure::COLOR => Ok((self.pen_color as i32).to_string()),
                }
            },
            Expr::EQ(expr1, expr2) => {
                let val1 = self.evaluate_expr(expr1)?.parse::<i32>().map_err(|_| format!("Cannot convert to integer"))?;
                let val2 = self.evaluate_expr(expr2)?.parse::<i32>().map_err(|_| format!("Cannot convert to integer"))?;
                let result = val1 == val2;
                println!("EQ result is {}", result);
                Ok((val1 == val2).to_string())
            },
            Expr::LT(expr1, expr2) => {
                let val1 = self.evaluate_expr(expr1)?.parse::<i32>().map_err(|_| format!("Cannot convert to integer"))?;
                let val2 = self.evaluate_expr(expr2)?.parse::<i32>().map_err(|_| format!("Cannot convert to integer"))?;
                let result = val1 < val2;
                println!("LT result is {}", result);
                Ok((val1 < val2).to_string())
            },
            Expr::GT(expr1, expr2) => {
                let val1 = self.evaluate_expr(expr1)?.parse::<i32>().map_err(|_| format!("Cannot convert to integer"))?;
                let val2 = self.evaluate_expr(expr2)?.parse::<i32>().map_err(|_| format!("Cannot convert to integer"))?;
                let result = val1 > val2;
                println!("GT result is {}", result);
                Ok(result.to_string())
            },
            Expr::NE(expr1, expr2) => {
                let val1 = self.evaluate_expr(expr1)?.parse::<i32>().map_err(|_| format!("Cannot convert to integer"))?;
                let val2 = self.evaluate_expr(expr2)?.parse::<i32>().map_err(|_| format!("Cannot convert to integer"))?;

                let result = val1 != val2;
                println!("NE result is {}", result);
                Ok(result.to_string())
            },
            Expr::AND(expr1, expr2, )=>{
                let val1 = self.evaluate_expr(expr1)?.parse::<bool>().map_err(|_| format!("Cannot convert to boolean"))?;
                let val2 = self.evaluate_expr(expr2)?.parse::<bool>().map_err(|_| format!("Cannot convert to boolean"))?;
                let result = val1 && val2;
                println!("AND result is {} val1 {} and val2 {}", result, val1, val2);
                Ok(result.to_string())
            },
            Expr::OR(expr1, expr2)=>{
                let val1 = self.evaluate_expr(expr1)?.parse::<bool>().map_err(|_| format!("Cannot convert to boolean"))?;
                let val2 = self.evaluate_expr(expr2)?.parse::<bool>().map_err(|_| format!("Cannot convert to boolean"))?;
                Ok((val1 || val2).to_string())
            },
            Expr::ADD(expr1, expr2)=>{
                let val1 = self.evaluate_expr(expr1)?.parse::<i32>().map_err(|_| format!("Cannot convert to integer"))?;
                let val2 = self.evaluate_expr(expr2)?.parse::<i32>().map_err(|_| format!("Cannot convert to integer"))?;
                Ok((val1 + val2).to_string())
            },
            Expr::SUB(expr1, expr2)=>{
                let val1 = self.evaluate_expr(expr1)?.parse::<i32>().map_err(|_| format!("Cannot convert to integer"))?;
                let val2 = self.evaluate_expr(expr2)?.parse::<i32>().map_err(|_| format!("Cannot convert to integer"))?;
                Ok((val1 - val2).to_string())
            },
            Expr::MUL(expr1, expr2)=>{
                let val1 = self.evaluate_expr(expr1)?.parse::<i32>().map_err(|_| format!("Cannot convert to integer"))?;
                let val2 = self.evaluate_expr(expr2)?.parse::<i32>().map_err(|_| format!("Cannot convert to integer"))?;
                Ok((val1 * val2).to_string())
            },
            Expr::DIV(expr1, expr2)=>{
                let val1 = self.evaluate_expr(expr1)?.parse::<i32>().map_err(|_| format!("Cannot convert to integer"))?;
                let val2 = self.evaluate_expr(expr2)?.parse::<i32>().map_err(|_| format!("Cannot convert to integer"))?;
                Ok((val1 / val2).to_string())
            },
            _ => Err(format!("Unimplemented expression: {:?}", expr)),
        }
    }

    
    fn move_forward(&mut self, distance: i32) {
        println!("Moving forward: {} (pen_down: {}) (pen_color: {})", distance, self.pen_down, self.pen_color);
        if self.pen_down {
            let start_x = self.x;
            let start_y = self.y;
            (self.x, self.y) = get_end_coordinates(self.x, self.y, self.heading, distance);
            match self.image.draw_simple_line(
                start_x,
                start_y,
                self.heading,
                distance,
                COLORS[self.pen_color as usize]
            ) {
                Ok(_) => println!("Line drawn from ({}, {}) to ({}, {})", start_x, start_y, self.x, self.y),
                Err(e) => eprintln!("Error drawing line: {}", e),
            }
        } else {
            (self.x, self.y) = get_end_coordinates(self.x, self.y, self.heading, distance);
            println!("Moved to ({}, {})", self.x, self.y);
        }
    }

    fn move_back(&mut self, distance: i32) {
        println!("Moving back: {} (pen_down: {}) (pen_color: {})", distance, self.pen_down, self.pen_color);
        if self.pen_down {
            let start_x = self.x;
            let start_y = self.y;
            (self.x, self.y) = get_end_coordinates(self.x, self.y, self.heading, -distance);
            match self.image.draw_simple_line(
                start_x,
                start_y,
                self.heading,
                -distance,
                COLORS[self.pen_color as usize]
            ) {
                Ok(_) => println!("Line drawn from ({}, {}) to ({}, {})", start_x, start_y, self.x, self.y),
                Err(e) => eprintln!("Error drawing line: {}", e),
            }
        } else {
            (self.x, self.y) = get_end_coordinates(self.x, self.y, self.heading, -distance);
            println!("Moved to ({}, {})", self.x, self.y);
        }
    }

    fn move_left(&mut self, distance: i32) {
        if self.pen_down{
            match self.image.draw_simple_line(
                self.x,
                self.y,
                (self.heading - 90 + 360) % 360,
                distance,
                COLORS[self.pen_color as usize]
            ) {
                Ok((new_x, new_y)) => {
                    self.x = new_x;
                    self.y = new_y;
                },
                Err(e) => eprintln!("Error drawing line: {}", e),
            }
        }else{
            (self.x, self.y) = get_end_coordinates(
                self.x,
                self.y,
                (self.heading - 90 + 360) % 360,
                distance
            )
        }
    }

    fn move_right(&mut self, distance: i32) {
        if self.pen_down{
            match self.image.draw_simple_line(
                self.x,
                self.y,
                (self.heading + 90) % 360,
                distance,
                COLORS[self.pen_color as usize]
            ) {
                Ok((new_x, new_y)) => {
                    self.x = new_x;
                    self.y = new_y;
                },
                Err(e) => eprintln!("Error drawing line: {}", e),
            }
        }else{
            (self.x, self.y) = get_end_coordinates(
                self.x,
                self.y,
                (self.heading + 90) % 360,
                distance
            )
        }
    }

    fn turn(&mut self, angle: i32){
        self.heading = self.heading + angle;
    }

    fn set_heading(&mut self, angle: i32){
        self.heading =  angle;
    }

    fn set_color(&mut self, color: u32){
        self.pen_color = color;
    }

    fn set_x(&mut self, location: i32){
        println!("Setting x from {} to {}", self.x, location);
        self.x = location;

    }

    fn set_y(&mut self, location: i32){
        self.y = location;
    }

}