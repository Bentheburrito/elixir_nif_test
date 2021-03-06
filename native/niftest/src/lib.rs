use rustler::{Encoder, Env, Error, Term};
// use rustler::types::atom::ok;

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler::rustler_export_nifs! {
    "Elixir.NifTest",
    [
        ("add", 2, add),
				("println", 1, println)
    ],
    None
}

fn add<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let num1: i64 = args[0].decode()?;
    let num2: i64 = args[1].decode()?;

    Ok((atoms::ok(), num1 + num2).encode(env))
}

fn println<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
	let message: String = args[0].decode()?;
	println!("{}", message);

	Ok(atoms::ok().encode(env))
}