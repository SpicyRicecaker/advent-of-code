use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug, Clone)]
struct Directory {
    parent: Option<Weak<RefCell<Entry>>>,
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

    let filesystem = Rc::new(RefCell::new(Entry::Directory(Directory {
        parent: None,
        name: "/".to_string(),
        contents: vec![],
    })));

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
                // until the next $ or end of line
                'inner: loop {
                    let Some(t) = iter.next() else {
                        break 'outer;
                    };

                    let line = t
                        .split_whitespace()
                        .map(String::from)
                        .collect::<Vec<String>>();

                    match line[0].as_str() {
                        "$" => {
                            inputs = line;
                            break 'inner;
                        }
                        "dir" => match *working_dir.borrow_mut() {
                            Entry::Directory(ref mut d) => d.contents.push(Rc::new(RefCell::new(
                                Entry::Directory(Directory {
                                    parent: Some(Rc::downgrade(&working_dir)),
                                    name: line[1].clone(),
                                    contents: vec![],
                                }),
                            ))),
                            _ => {
                                panic!()
                            }
                        },
                        // this is a file
                        size => match *working_dir.borrow_mut() {
                            Entry::Directory(ref mut d) => {
                                d.contents.push(Rc::new(RefCell::new(Entry::File(File {
                                    name: line[1].clone(),
                                    size: size.parse::<usize>().unwrap(),
                                }))))
                            }
                            _ => {
                                panic!()
                            }
                        },
                    }
                }
            }
            "cd" => {
                match inputs[2].as_str() {
                    ".." => {
                        // could break if we're at root, or at a directory whose
                        // parent has been deleted
                        // adding mutability to a tree just gives 10000 bugs,
                        // arena is preferred if there is no deletion,
                        // and even with deletion it's fine as long as you're ok
                        // with dangling refs and no cleanup, cleanup being O(n)
                        working_dir = match &*working_dir.clone().borrow() {
                            Entry::Directory(d) => d.parent.clone().unwrap().upgrade().unwrap(),
                            _ => panic!(),
                        }
                    }
                    dir => {
                        working_dir = match &*working_dir.clone().borrow() {
                            Entry::Directory(ref d) => d
                                .contents
                                .iter()
                                .find(|d| {
                                    if let Entry::Directory(c_dir) = &*d.borrow() {
                                        c_dir.name == dir
                                    } else {
                                        false
                                    }
                                })
                                .expect("could not find dir")
                                .clone(),
                            _ => {
                                panic!()
                            }
                        }
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
    // // traverse the entire tree
    // dbg!(res);

    // clone because otherwise we might lose nodes while recursing?
    let mut v = vec![];
    recurse(filesystem.clone(), &mut v);

    // v.sort();

    // v.into_iter().rev().find(|n| )

    println!("{}", v.into_iter().filter(|&n| n <= 100_000).sum::<usize>());
}

fn recurse(entry: Rc<RefCell<Entry>>, v: &mut Vec<usize>) -> usize {
    // find sum of directories with a total size of at most 100000

    let mut sum: usize = 0;

    let Entry::Directory(d) = &*entry.borrow() else {
        panic!();
    };

    for content in d.contents.iter() {
        match &*content.borrow() {
            Entry::Directory(_) => {
                sum += recurse(content.clone(), v);
            }
            Entry::File(f) => {
                sum += f.size;
            }
        }
    }

    v.push(sum);

    sum
}
