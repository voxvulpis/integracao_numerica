use std::fs;
use std::io;

mod formulas;

fn main() -> io::Result<()>{
    let from = 0.0;
    let to = 100.0;
    let n = 120;
    let h = (to - from)/n as f64;

    let matrix: Vec<Vec<f64>> = formulas::expressions::wrapper(from, to, n);
    let mut results: Vec<Vec<f64>> = Vec::new();

    let mut output: String = String::new();

    for i in 1..matrix.len(){
        results.push(formulas::compostas::wrapper(&matrix[i], &h));
    }


    for i in 0..matrix.len(){
        match i {
            0 => output += "Valores de x,",
            1 => output += "Valores de F(x) Primeiro Grau,",
            2 => output += "Valores de F(x) Segundo Grau,",
            3 => output += "Valores de F(x) Terceiro Grau,",
            4 => output += "Valores de F(x) Quarto Grau,",
            5 => output += "Valores de F(x) Trigonometrico,",
            _ => break,
        }

        for j in 0..matrix[i].len(){
            let num = matrix[i][j];
            let str = format!("{:.10}", num);
            output += &str;
            output += ",";
        }
        output += ",";

        if i == 0{
            output += ",,,";
        }
        else{
            for k in 0..results[i - 1].len(){
                let num = results[i - 1][k];
                let str = format!("{:.10}", num);
                output += &str;
                output += ",";
            }
        }
        output += "\n";

    }

    print!("{}", output);

    let path = "resultado_from".to_owned() + &from.to_string() + "_to" + &to.to_string() + "_n" + &n.to_string() + ".csv";

    fs::write(path, output)?;
    Ok(())
}
