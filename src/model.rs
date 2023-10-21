use std::{
    fs::OpenOptions,
    io::{BufReader, Read, Write},
};

#[allow(unused)]
#[derive(Debug)]
pub struct Item {
    todo: String,
    filepath: String,
}

#[allow(unused)]
impl Item {
    pub fn new() -> Self {
        let filepath = "todo.txt".to_string();
        let todofile = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&filepath)
            .expect("Failed to open file");
        let mut contents = String::new();
        let mut buf_reader = BufReader::new(&todofile);
        buf_reader
            .read_to_string(&mut contents)
            .expect("Failed to read file");
        let todo = contents;
        Self { todo, filepath }
    }

    pub fn add_item(&mut self, item: String) {
        //添加类似1.[ ] item的格式
        let index = self.get_index();
        let time = chrono::prelude::Local::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        let mut item = format!("{}.[ ] {} {}\n", index, item, time);
        self.todo.push_str(&mut item);
        self.write();
    }

    fn get_index(&self) -> usize {
        self.todo.split('\n').count()
    }

    fn write(&self) {
        let is_truncate = self.todo.is_empty();
        let mut todofile = OpenOptions::new()
            .write(true)
            .truncate(is_truncate)
            .open(&self.filepath)
            .expect("Failed to open file");
        todofile
            .write_all(self.todo.as_bytes())
            .expect("Failed to write to file");
    }

    pub fn list_items(&self) {
        println!("{}", self.todo);
    }

    pub fn done_items(&mut self, no: usize) {
        //将指定index对应的item的[ ]替换为[x]
        //先将todo按行分割
        let mut lines: Vec<&str> = self.todo.split('\n').collect();
        //再将指定行的[ ]替换为[x]
        let mut line = lines[no - 1].to_string();
        if line.contains("[ ]") {
            line = line.replace("[ ]", "[x]");
        } else if line.contains("[x]") {
            line = line.replace("[x]", "[ ]");
        }
        lines[no - 1] = &line;
        //再将lines重新组合为todo
        let mut todo = String::new();
        for line in lines {
            todo.push_str(line);
            todo.push('\n');
        }
        self.todo = todo;
        self.write();
    }

    pub fn clear_items(&mut self) {
        //清空todo
        self.todo = String::new();
        self.write();
    }
}
