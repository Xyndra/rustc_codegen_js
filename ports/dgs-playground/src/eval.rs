use dgs::{
    circle, ellipse, eq, eq_param, line as line_fn, point, point_at, polygon, semicircle, Color,
    Object, PointRef, Scene, SemicircleDir, Theme, Viewport,
};

#[derive(Debug, Clone)]
enum Ast {
    Call { name: String, args: Vec<Arg> },
    Method { obj: Box<Ast>, name: String, args: Vec<Arg> },
    Tuple { items: Vec<Ast> },
    Ident(String),
    Lit(Lit),
}

#[derive(Debug, Clone)]
enum Arg {
    Named(String, Box<Ast>),
    Pos(Box<Ast>),
}

impl Arg {
    fn as_ast(&self) -> &Ast {
        match self {
            Arg::Pos(a) => a,
            Arg::Named(_, v) => v,
        }
    }
}

#[derive(Debug, Clone)]
enum Lit {
    Str(String),
    Num(f64),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Str(String),
    Num(f64),
    Bool(bool),
    Ident(String),
    LParen,
    RParen,
    Comma,
    Dot,
    Colon,
    Eof,
}

struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(s: &str) -> Self {
        Lexer {
            chars: s.chars().collect(),
            pos: 0,
        }
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn skip_ws(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn next(&mut self) -> Result<Token, String> {
        self.skip_ws();
        if self.pos >= self.chars.len() {
            return Ok(Token::Eof);
        }
        let c = self.chars[self.pos];
        match c {
            '(' => {
                self.pos += 1;
                Ok(Token::LParen)
            }
            ')' => {
                self.pos += 1;
                Ok(Token::RParen)
            }
            ',' => {
                self.pos += 1;
                Ok(Token::Comma)
            }
            '.' => {
                self.pos += 1;
                Ok(Token::Dot)
            }
            ':' => {
                self.pos += 1;
                Ok(Token::Colon)
            }
            '"' => self.parse_str(),
            '-' if self
                .peek_next()
                .map(|n| n.is_ascii_digit())
                .unwrap_or(false) =>
            {
                self.parse_num()
            }
            c if c.is_ascii_digit() => self.parse_num(),
            c if c.is_alphabetic() || c == '_' => self.parse_ident(),
            c => Err(format!("unexpected character '{}'", c)),
        }
    }

    fn parse_str(&mut self) -> Result<Token, String> {
        self.pos += 1;
        let mut s = String::new();
        while self.pos < self.chars.len() && self.chars[self.pos] != '"' {
            if self.chars[self.pos] == '\\' {
                self.pos += 1;
                match self.chars.get(self.pos) {
                    Some(&'n') => s.push('\n'),
                    Some(&'t') => s.push('\t'),
                    Some(&'r') => s.push('\r'),
                    Some(&'"') => s.push('"'),
                    Some(&'\\') => s.push('\\'),
                    Some(&c) => s.push(c),
                    None => return Err("unterminated string".into()),
                }
            } else {
                s.push(self.chars[self.pos]);
            }
            self.pos += 1;
        }
        if self.pos >= self.chars.len() {
            return Err("unterminated string".into());
        }
        self.pos += 1;
        Ok(Token::Str(s))
    }

    fn parse_num(&mut self) -> Result<Token, String> {
        let start = self.pos;
        if self.chars[self.pos] == '-' {
            self.pos += 1;
        }
        while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        if self.pos < self.chars.len() && self.chars[self.pos] == '.' {
            self.pos += 1;
            while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }
        let s: String = self.chars[start..self.pos].iter().collect();
        match s.parse::<f64>() {
            Ok(n) => Ok(Token::Num(n)),
            Err(_) => Err(format!("bad number '{}'", s)),
        }
    }

    fn parse_ident(&mut self) -> Result<Token, String> {
        let start = self.pos;
        while self.pos < self.chars.len()
            && (self.chars[self.pos].is_alphanumeric() || self.chars[self.pos] == '_')
        {
            self.pos += 1;
        }
        let s: String = self.chars[start..self.pos].iter().collect();
        match s.as_str() {
            "true" => Ok(Token::Bool(true)),
            "false" => Ok(Token::Bool(false)),
            _ => Ok(Token::Ident(s)),
        }
    }

    fn tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut ts = Vec::new();
        loop {
            let t = self.next()?;
            if t == Token::Eof {
                break;
            }
            ts.push(t);
        }
        Ok(ts)
    }
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(ts: Vec<Token>) -> Self {
        Parser { tokens: ts, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    /// The token AFTER the current one (for `name:` lookahead).
    fn peek_next(&self) -> &Token {
        self.tokens.get(self.pos + 1).unwrap_or(&Token::Eof)
    }

    fn cur(&self) -> Token {
        self.peek().clone()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, t: Token) -> Result<(), String> {
        if self.cur() != t {
            return Err(format!("expected {:?}, found {:?}", t, self.cur()));
        }
        self.advance();
        Ok(())
    }

    fn parse_program(&mut self) -> Result<Vec<Ast>, String> {
        let mut exprs = Vec::new();
        loop {
            match self.cur() {
                Token::Eof => break,
                _ => {
                    let expr = self.parse_expr()?;
                    exprs.push(expr);
                    if self.cur() == Token::Eof {
                        break;
                    }
                }
            }
        }
        Ok(exprs)
    }

    fn parse_expr(&mut self) -> Result<Ast, String> {
        self.parse_member_chain()
    }

    fn parse_member_chain(&mut self) -> Result<Ast, String> {
        let mut node = self.parse_primary()?;
        while self.cur() == Token::Dot {
            self.advance();
            let name = match self.cur() {
                Token::Ident(n) => n,
                other => {
                    return Err(format!("expected identifier after '.', found {:?}", other));
                }
            };
            self.advance();
            let args = if self.cur() == Token::LParen {
                self.parse_call_args()?
            } else {
                vec![]
            };
            node = Ast::Method {
                obj: Box::new(node),
                name,
                args,
            };
        }
        Ok(node)
    }

    fn parse_primary(&mut self) -> Result<Ast, String> {
        match self.cur() {
            Token::LParen => {
                self.advance();
                if self.cur() == Token::RParen {
                    self.advance();
                    return Ok(Ast::Tuple { items: vec![] });
                }
                let first = self.parse_expr()?;
                if self.cur() == Token::RParen {
                    self.advance();
                    return Ok(first);
                }
                self.expect(Token::Comma)?;
                let mut items = vec![first];
                loop {
                    // Tolerate a trailing comma: `(a, b,)`.
                    if self.cur() == Token::RParen {
                        break;
                    }
                    items.push(self.parse_expr()?);
                    if self.cur() == Token::Comma {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.expect(Token::RParen)?;
                Ok(Ast::Tuple { items })
            }
            Token::Str(s) => {
                self.advance();
                Ok(Ast::Lit(Lit::Str(s)))
            }
            Token::Num(n) => {
                self.advance();
                Ok(Ast::Lit(Lit::Num(n)))
            }
            Token::Bool(b) => {
                self.advance();
                Ok(Ast::Lit(Lit::Bool(b)))
            }
            Token::Ident(n) => {
                self.advance();
                if self.cur() == Token::LParen {
                    let args = self.parse_call_args()?;
                    Ok(Ast::Call { name: n, args })
                } else {
                    Ok(Ast::Ident(n))
                }
            }
            other => Err(format!("unexpected token {:?}", other)),
        }
    }

    fn parse_call_args(&mut self) -> Result<Vec<Arg>, String> {
        self.expect(Token::LParen)?;
        let mut args = Vec::new();
        while self.cur() != Token::RParen && self.cur() != Token::Eof {
            if let Token::Ident(name) = self.cur() {
                if self.peek_next().clone() == Token::Colon {
                    self.advance();
                    self.advance();
                    let val = self.parse_expr()?;
                    args.push(Arg::Named(name.clone(), Box::new(val)));
                    if self.cur() == Token::Comma {
                        self.advance();
                    }
                    continue;
                }
            }
            let val = self.parse_expr()?;
            args.push(Arg::Pos(Box::new(val)));
            if self.cur() == Token::Comma {
                self.advance();
            }
        }
        self.expect(Token::RParen)?;
        Ok(args)
    }
}

fn parse_program(src: &str) -> Result<Vec<Ast>, String> {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokens()?;
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}

fn unwrap_chain(ast: &Ast) -> (&Ast, Vec<(String, Vec<Arg>)>) {
    let mut methods: Vec<(String, Vec<Arg>)> = Vec::new();
    let mut cur = ast;
    loop {
        match cur {
            Ast::Method { obj, name, args } => {
                methods.push((name.clone(), args.clone()));
                cur = obj;
            }
            _ => break,
        }
    }
    methods.reverse();
    (cur, methods)
}

fn arg_ast(a: Option<&Arg>) -> Option<&Ast> {
    a.map(|a| a.as_ast())
}

fn arg_lit<'a>(args: Option<&'a Arg>, what: &str) -> Result<&'a Ast, String> {
    match args {
        Some(Arg::Pos(a)) | Some(Arg::Named(_, a)) => Some(&**a),
        None => None,
    }
    .ok_or_else(|| format!("{} must be a literal, got {:?}", what, args))
}

fn arg_as_str(args: Option<&Arg>, what: &str) -> Result<String, String> {
    let a = arg_lit(args, what)?;
    match a {
        Ast::Lit(Lit::Str(s)) => Ok(s.clone()),
        _ => Err(format!("{} must be a string, got {:?}", what, args)),
    }
}

fn arg_as_num(args: Option<&Arg>, what: &str) -> Result<f64, String> {
    let a = arg_lit(args, what)?;
    match a {
        Ast::Lit(Lit::Num(n)) => Ok(*n),
        _ => Err(format!("{} must be a number, got {:?}", what, args)),
    }
}

fn arg_as_bool(args: Option<&Arg>, what: &str) -> Result<bool, String> {
    let a = arg_lit(args, what)?;
    match a {
        Ast::Lit(Lit::Bool(b)) => Ok(*b),
        _ => Err(format!("{} must be a boolean, got {:?}", what, args)),
    }
}

fn arg_as_tuple(items: &[Ast]) -> Result<(f64, f64), String> {
    if items.len() != 2 {
        return Err(format!("expected a 2-element tuple, got {}", items.len()));
    }
    let x = match &items[0] {
        Ast::Lit(Lit::Num(n)) => *n,
        _ => return Err("tuple x must be a number".into()),
    };
    let y = match &items[1] {
        Ast::Lit(Lit::Num(n)) => *n,
        _ => return Err("tuple y must be a number".into()),
    };
    Ok((x, y))
}

fn parse_color_arg(arg: Option<&Arg>) -> Result<Color, String> {
    let a = arg_ast(arg);
    match a {
        Some(Ast::Lit(Lit::Str(s))) => {
            Color::parse(s).ok_or_else(|| format!("invalid color '{}'", s))
        }
        Some(Ast::Method { name, args, .. }) => match name.as_str() {
            "color" => parse_color_arg(args.get(0)),
            "rgb" => {
                let r = arg_as_num(args.get(0), "rgb r")?;
                let g = arg_as_num(args.get(1), "rgb g")?;
                let b = arg_as_num(args.get(2), "rgb b")?;
                if !(0.0..=255.0).contains(&r)
                    || !(0.0..=255.0).contains(&g)
                    || !(0.0..=255.0).contains(&b)
                {
                    return Err("rgb components must be 0..255".into());
                }
                Ok(Color::rgb(r as u8, g as u8, b as u8))
            }
            other => Err(format!("unknown color method '{}'", other)),
        },
        None => Err("color is required".into()),
        other => Err(format!("unexpected color argument {:?}", other)),
    }
}

fn parse_dir(arg: Option<&Ast>) -> Result<SemicircleDir, String> {
    match arg {
        Some(Ast::Lit(Lit::Str(s))) => {
            SemicircleDir::parse(s).ok_or_else(|| format!("invalid dir '{}'", s))
        }
        None => Ok(SemicircleDir::CounterClockwise),
        _ => Err(format!("dir must be a string, got {:?}", arg)),
    }
}

fn parse_point_ref(arg: Option<&Arg>) -> Result<PointRef, String> {
    let a = arg_ast(arg);
    match a {
        Some(Ast::Lit(Lit::Str(s))) => Ok(PointRef::Named(s.clone())),
        Some(Ast::Tuple { items }) => {
            let (x, y) = arg_as_tuple(items)?;
            Ok(PointRef::Coords(x, y))
        }
        _ => Err("point reference must be a string name or (x, y) tuple".into()),
    }
}

fn apply_modifiers(mut obj: Object, modifiers: &[(String, Vec<Arg>)]) -> Result<Object, String> {
    for (mname, margs) in modifiers {
        obj = match mname.as_str() {
            "with_color" => obj.with_color(parse_color_arg(margs.get(0))?),
            "with_size" => obj.with_size(arg_as_num(margs.get(0), "size")?),
            "with_stroke" => obj.with_stroke(arg_as_num(margs.get(0), "stroke")?),
            "with_fill" => obj.with_fill(parse_color_arg(margs.get(0))?),
            "with_rotation" => obj.with_rotation(arg_as_num(margs.get(0), "rotation")?),
            "with_dir" => obj.with_dir(parse_dir(margs.get(0).map(|a| a.as_ast()))?),
            other => return Err(format!("unknown modifier '{}'", other)),
        };
    }
    Ok(obj)
}

fn eval_shape(ast: &Ast) -> Result<Object, String> {
    let (base, methods) = unwrap_chain(ast);
    let (name, args) = match base {
        Ast::Call { name, args } => (name.clone(), args.clone()),
        _ => return Err("expected a function call".into()),
    };

    let mut obj = match name.as_str() {
        "point" => {
            let n = arg_as_str(args.get(0), "name")?;
            let x = arg_as_num(args.get(1), "x")?;
            let y = arg_as_num(args.get(2), "y")?;
            point(n, x, y)
        }
        "point_at" => {
            let x = arg_as_num(args.get(0), "x")?;
            let y = arg_as_num(args.get(1), "y")?;
            point_at(x, y)
        }
        "line" => {
            let from = parse_point_ref(args.get(0))?;
            let to = parse_point_ref(args.get(1))?;
            line_fn(from, to)
        }
        "circle" => {
            let center = parse_point_ref(args.get(0))?;
            let radius = arg_as_num(args.get(1), "radius")?;
            circle(center, radius)
        }
        "arc" => {
            let center = parse_point_ref(args.get(0))?;
            let radius = arg_as_num(args.get(1), "radius")?;
            let start = arg_as_num(args.get(2), "start")?;
            let end = arg_as_num(args.get(3), "end")?;
            dgs::arc(center, radius, start, end)
        }
        "polygon" => {
            let mut points = Vec::new();
            for a in args.iter() {
                match a {
                    Arg::Named(_, _) => {
                        return Err("polygon uses positional arguments only".into())
                    }
                    Arg::Pos(_) => {
                        points.push(parse_point_ref(Some(a))?);
                    }
                }
            }
            polygon(points)
        }
        "ellipse" => {
            let center = parse_point_ref(args.get(0))?;
            let rx = arg_as_num(args.get(1), "rx")?;
            let ry = arg_as_num(args.get(2), "ry")?;
            ellipse(center, rx, ry)
        }
        "semicircle" => {
            let mut dir_arg = None;
            for a in args.iter() {
                if let Arg::Named(k, v) = a {
                    if k.as_str() == "dir" {
                        dir_arg = Some(v);
                        break;
                    }
                }
            }
            let dir = match dir_arg {
                Some(v) => parse_dir(Some(v))?,
                None => SemicircleDir::CounterClockwise,
            };
            let from = parse_point_ref(args.get(0))?;
            let to = parse_point_ref(args.get(1))?;
            semicircle(from, to).with_dir(dir)
        }
        "eq" => {
            let expr = arg_as_str(args.get(0), "expression")?;
            eq(&expr)
        }
        "eq_param" => {
            let xe = arg_as_str(args.get(0), "x expr")?;
            let ye = arg_as_str(args.get(1), "y expr")?;
            eq_param(&xe, &ye)
        }
        other => {
            return Err(format!(
                "unknown command '{}'; supported: point, point_at, line, circle, arc, polygon, ellipse, semicircle, eq, eq_param, push, scene, view, theme, background, grid, axes, curve",
                other
            ))
        }
    };

    for a in &args {
        if let Arg::Named(k, v) = a {
            match k.as_str() {
                "color" => obj = obj.with_color(parse_color_arg(Some(a))?),
                "size" => obj = obj.with_size(arg_as_num(Some(a), "size")?),
                "stroke" => obj = obj.with_stroke(arg_as_num(Some(a), "stroke")?),
                "fill" => obj = obj.with_fill(parse_color_arg(Some(a))?),
                "rotation" => obj = obj.with_rotation(arg_as_num(Some(a), "rotation")?),
                "dir" => obj = obj.with_dir(parse_dir(Some(v))?),
                _ => return Err(format!("unknown keyword argument '{}'", k)),
            }
        }
    }

    apply_modifiers(obj, &methods)
}

struct Context {
    scene: Option<Scene>,
}

impl Context {
    fn new() -> Self {
        Context {
            scene: Some(Scene::new(Viewport::new(-5.0, -5.0, 5.0, 5.0, 800.0, 600.0))),
        }
    }
}

fn interpret_stmt(ast: &Ast, ctx: &mut Context) -> Result<(), String> {
    let (base, methods) = unwrap_chain(ast);
    let (name, args) = match base {
        Ast::Call { name, args } => {
            // Only `scene()` takes method calls (`.push(...)`); every other
            // command rejects modifiers so typos surface as errors.
            if !methods.is_empty() && name != "scene" {
                return Err("scene commands cannot take modifiers".into());
            }
            (name.clone(), args.clone())
        }
        Ast::Ident(s) => return Err(format!("unexpected identifier '{}'", s)),
        other => return Err(format!("expected a function call, found {:?}", other)),
    };

    match name.as_str() {
        "push" => {
            let arg = args
                .get(0)
                .ok_or_else(|| "push() requires an argument".to_string())?;
            let obj = eval_shape(arg.as_ast())?;
            ctx.scene = Some(ctx.scene.take().unwrap().push(obj));
        }
        "scene" => {
            if !args.is_empty() {
                return Err("scene() takes no arguments".into());
            }
            let mut new_scene = Scene::new(Viewport::new(-5.0, -5.0, 5.0, 5.0, 800.0, 600.0));
            let mut pushed = false;
            for (mname, margs) in &methods {
                if mname == "push" {
                    if margs.is_empty() {
                        return Err(".push() needs an argument".into());
                    }
                    let obj = eval_shape(margs[0].as_ast())?;
                    new_scene = new_scene.push(obj);
                    pushed = true;
                } else {
                    return Err(format!(
                        "unexpected modifier '.{}()' after scene()",
                        mname
                    ));
                }
            }
            if !pushed {
                return Err("scene() must be followed by .push(...) with a shape".into());
            }
            ctx.scene = Some(new_scene);
        }
        "view" => {
            let x1 = arg_as_num(args.get(0), "x1")?;
            let y1 = arg_as_num(args.get(1), "y1")?;
            let x2 = arg_as_num(args.get(2), "x2")?;
            let y2 = arg_as_num(args.get(3), "y2")?;
            let w = arg_as_num(args.get(4), "width").unwrap_or(800.0);
            let h = arg_as_num(args.get(5), "height").unwrap_or(600.0);
            ctx.scene = Some(
                ctx.scene
                    .take()
                    .unwrap()
                    .viewport(Viewport::new(x1, y1, x2, y2, w, h)),
            );
        }
        "theme" => {
            let s = arg_as_str(args.get(0), "theme")?;
            let theme = match s.as_str() {
                "light" => Theme::light(),
                "dark" => Theme::dark(),
                _ => return Err(format!("theme must be 'light' or 'dark', got '{}'", s)),
            };
            ctx.scene = Some(ctx.scene.take().unwrap().theme(theme));
        }
        "background" => {
            let c = parse_color_arg(args.get(0))?;
            let theme = Theme::custom(
                c,
                Color::rgb(224, 224, 224),
                Color::rgb(0, 0, 0),
                Color::rgb(0, 0, 0),
                Color::rgb(0, 0, 0),
            );
            ctx.scene = Some(ctx.scene.take().unwrap().theme(theme));
        }
        "grid" => {
            ctx.scene = Some(
                ctx.scene
                    .take()
                    .unwrap()
                    .grid(arg_as_bool(args.get(0), "grid")?),
            );
        }
        "axes" => {
            ctx.scene = Some(
                ctx.scene
                    .take()
                    .unwrap()
                    .axes(arg_as_bool(args.get(0), "axes")?),
            );
        }
        "curve" => {
            let expr = arg_as_str(args.get(0), "expression")?;
            let t_min = arg_as_num(args.get(1), "t_min").ok();
            let t_max = arg_as_num(args.get(2), "t_max").ok();
            ctx.scene = Some(ctx.scene.take().unwrap().push(Object::Curve {
                expr_str: expr,
                var_name: "x".into(),
                t_min,
                t_max,
                samples: None,
                tolerance: None,
                color: None,
                stroke: None,
            }));
        }
        "eq" => {
            let expr = arg_as_str(args.get(0), "expression")?;
            ctx.scene = Some(ctx.scene.take().unwrap().push(eq(&expr)));
        }
        "eq_param" => {
            let xe = arg_as_str(args.get(0), "x expr")?;
            let ye = arg_as_str(args.get(1), "y expr")?;
            ctx.scene = Some(ctx.scene.take().unwrap().push(eq_param(&xe, &ye)));
        }
        other => {
            return Err(format!(
                "unknown command '{}'; supported: point, point_at, line, circle, arc, polygon, ellipse, semicircle, eq, eq_param, push, scene, view, theme, background, grid, axes, curve",
                other
            ))
        }
    }
    Ok(())
}

pub fn render(code: &str) -> Result<String, String> {
    let stmts = parse_program(code)?;
    let mut ctx = Context::new();
    for stmt in stmts {
        if let Err(e) = interpret_stmt(&stmt, &mut ctx) {
            return Err(e);
        }
    }
    Ok(ctx.scene.take().unwrap().to_svg())
}
