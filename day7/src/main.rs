struct File {
  name: String,
  size: i32,
}

struct Folder {
  sub_folders: Vec<Folder>,
  files: Vec<File>,
  name: String,
  super_folder: Folder,
}

fn parse_command(root: Folder, line: &str)
{
  let parts = line.split(" ");
  parts.next(); //we skip the $ at the beginning

  let command = parts.next();

  if command.eq("cd")
  {
    
  }
  else if command.eq("ls")
  {

  }
}

fn main() {
  let input_str = include_str!("input_test.txt");
  let lines = input_str.lines();
  
  let root = Folder(sub_folders= vec![], files=[], name="/");
  line.next();

  for line in lines.chars()
  {
    if line[0] == '$'
    {
      root = parse_command(root, line);
    }
    else
    {
      root = parse_file(root, line);
    }
  }
}
