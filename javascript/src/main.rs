use js::{eval, Declare, Expression, Instruction, Primitive, Lookup,Scope, Scopes};

fn main() -> Result<(), String> {
    let mut global = Scope::new();
    let mut local = Scopes::new();

    // let x = 1.0
    let declare_x = Expression::Instruction(
        Box::new(
            Instruction::Declare(
                Declare::Let(
                    "x".to_string(),
                    Some(
                        Expression::Primitive(Primitive::Number(1.0))
                    ),
                ),
            )
        )
    );

    // let y = x
    let declare_y = Expression::Instruction(
        Box::new(
            Instruction::Declare(
                Declare::Let(
                    "y".to_string(),
                    Some(
                        Expression::Instruction(
                            Box::new(Instruction::Lookup(Lookup("x".to_string())))
                        )
                    ),
                ),
            )
        )
    );

    let mut expression = vec![
        declare_x,
        declare_y,
    ];
    eval(&mut global, &mut local, &mut expression)?;

    dbg!(global);
    Ok(())
}
