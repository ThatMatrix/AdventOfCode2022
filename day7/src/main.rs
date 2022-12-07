struct File {
    name: String,
    size: i32,
}

struct Folder<'a> {
    sub_folders: Vec<Box<Folder<'a>>>,
    files: Vec<File>,
    name: String,
    super_folder: Option<Box<&'a Folder<'a>>>,
    size : i32,
}

fn main() {
    let input_str = include_str!("input_test.txt");
    let mut lines = input_str.lines();

    let mut root = Folder{sub_folders: vec![], files:vec![], name:"/".to_string(), size : 0, super_folder:None};
    let save = root;
    lines.next(); //skip the first cd /
    let mut line = lines.next(); //parse the first command line

    while line != None
    {
        let mut parts = line.unwrap().split(" ");
        parts.next(); //we skip the $ at the beginning

        let command = parts.next();

        if command.unwrap().eq("cd")
        {
            let path = parts.next();
            for f in root.sub_folders
            {
                if f.name.eq(path.unwrap())
                {
                    root = *f;
                }
            }
            root = *(*root.super_folder.unwrap());
            line = lines.next();
        }
        else if command.unwrap().eq("ls")
        {
            line = lines.next();
            while line != None && line.expect("NaN").chars().nth(0) != Some('$')
            {
                let mut parts = line.unwrap().split(" ");
                let p1 = parts.next();
                if p1.unwrap().eq("dir")
                {
                    root.sub_folders.push(Box::new(Folder{sub_folders:vec![], files:vec![], name:parts.next().unwrap().to_string(), size:0, super_folder:Some(Box::new(&root))}));
                }
                else
                {
                    root.files.push(File{size:p1.expect("p1 NaN").parse().unwrap(), name: parts.next().unwrap().to_string()});
                }
            }
        }
    }

    println!("arrived at the end {}", save.name);
}
