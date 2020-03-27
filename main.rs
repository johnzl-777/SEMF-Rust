mod semf;

fn main() {
    // Binding Energy of Palladium 
    println!("{}", semf::semi_emp_bind_eng(106, 46, semf::SemiEmpConsts::linear_fit_1()));
}


