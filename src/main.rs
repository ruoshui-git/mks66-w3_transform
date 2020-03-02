mod graphics;

use graphics::parser::DWScript;

fn main() {
    let mut script = DWScript::new("script");
    script.do_parse();
    
}