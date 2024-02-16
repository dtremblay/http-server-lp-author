fn main() {

  let art : &str = r#"
 █     █░ ▄▄▄        ██████  ███▄ ▄███▓▓█████ ▓█████▄   ▄████ ▓█████ 
▓█░ █ ░█░▒████▄    ▒██    ▒ ▓██▒▀█▀ ██▒▓█   ▀ ▒██▀ ██▌ ██▒ ▀█▒▓█   ▀ 
▒█░ █ ░█ ▒██  ▀█▄  ░ ▓██▄   ▓██    ▓██░▒███   ░██   █▌▒██░▄▄▄░▒███   
░█░ █ ░█ ░██▄▄▄▄██   ▒   ██▒▒██    ▒██ ▒▓█  ▄ ░▓█▄   ▌░▓█  ██▓▒▓█  ▄ 
░░██▒██▓  ▓█   ▓██▒▒██████▒▒▒██▒   ░██▒░▒████▒░▒████▓ ░▒▓███▀▒░▒████▒
░ ▓░▒ ▒   ▒▒   ▓▒█░▒ ▒▓▒ ▒ ░░ ▒░   ░  ░░░ ▒░ ░ ▒▒▓  ▒  ░▒   ▒ ░░ ▒░ ░
  ▒ ░ ░    ▒   ▒▒ ░░ ░▒  ░ ░░  ░      ░ ░ ░  ░ ░ ▒  ▒   ░   ░  ░ ░  ░
  ░   ░    ░   ▒   ░  ░  ░  ░      ░      ░    ░ ░  ░ ░ ░   ░    ░   
    ░          ░  ░      ░         ░      ░  ░   ░          ░    ░  ░
                                               ░                     
  "#;
  println!("{}", art);
  // hello variable is immutable
  let hello : &str = "Hello WasmEdge!";

  // the howdy variable can be manipulated - like a Java StringBuffer
  let mut howdy : String = hello.replace("Hello", "Howdy");
  println!("{}", hello);
  println!("{}", howdy);
  
  howdy.push_str(" -- from Texas");
  println!("{}", howdy);
}
