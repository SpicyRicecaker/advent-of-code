use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug, Clone)]
struct Directory {
    parent: Option<Weak<RefCell<Directory>>>,
    name: String,
    contents: Vec<Rc<RefCell<Entry>>>,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
enum Entry {
    Directory(Directory),
    File(File),
}

fn main() {
    // the first line is always slash?

    let filesystem = Rc::new(RefCell::new(Directory {
        parent: None,
        name: "/".to_string(),
        contents: vec![],
    }));

    let mut working_dir = filesystem.clone();

    let lines = std::fs::read_to_string("seven.txt").unwrap();
    let mut iter = lines.lines().skip(1);

    let mut inputs = iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();

    'outer: loop {
        match inputs[1].as_str() {
            "ls" => {
                println!("ls called");
                // until the next $ or end of line
                'inner: loop {
                    let Some(t) = iter.next() else {
                        break 'outer;
                    };

                    let line = t
                        .split_whitespace()
                        .map(String::from)
                        .collect::<Vec<String>>();

                    dbg!(&line);

                    match line[0].as_str() {
                        "$" => {
                            inputs = line;
                            println!("breakin");
                            break 'inner;
                        }
                        "dir" => {
                            working_dir.borrow_mut().contents.push(Rc::new(RefCell::new(
                                Entry::Directory(Directory {
                                    parent: Some(Rc::downgrade(&working_dir)),
                                    name: line[1].clone(),
                                    contents: vec![],
                                }),
                            )));
                        }
                        // this is a file
                        size => {
                            working_dir.borrow_mut().contents.push(Rc::new(RefCell::new(
                                Entry::File(File {
                                    name: line[1].clone(),
                                    size: size.parse::<usize>().unwrap(),
                                }),
                            )));
                        }
                    }
                }
            }
            "cd" => {
                match inputs[2].as_str() {
                    ".." => {
                        println!("inside cd ..: {:#?}", inputs);
                        println!(
                            "our parent directory is: {:#?}",
                            working_dir.borrow().parent.as_ref().unwrap().upgrade()
                        );
                        // could break if we're at root, or at a directory whose
                        // parent has been deleted
                        // adding mutability to a tree just gives 10000 bugs,
                        // arena is preferred if there is no deletion,
                        // and even with deletion it's fine as long as you're ok
                        // with dangling refs and no cleanup, cleanup being O(n)
                        working_dir = working_dir
                            .clone()
                            .borrow()
                            .parent
                            .as_ref()
                            .expect("expected self to have parent")
                            .upgrade()
                            .unwrap();
                    }
                    dir => {
                        println!("inside cd dir: {:#?}", inputs);
                        working_dir = Rc::new(RefCell::new(
                            working_dir
                                .clone()
                                .borrow()
                                .contents
                                .iter()
                                .find_map(|d| {
                                    if let Entry::Directory(c_dir) = &*d.borrow() {
                                        if c_dir.name == dir {
                                            Some(c_dir.clone())
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .expect("could not find dir"),
                        ));
                    }
                }

                let Some(t) = iter.next() else {
                    break 'outer;
                };

                inputs = t
                    .split_whitespace()
                    .map(String::from)
                    .collect::<Vec<String>>();
            }
            _ => {
                panic!("should not happen")
            }
        }
    }
    dbg!(filesystem);
}
