use std::env;

use inotify::{EventMask, Inotify, WatchMask};

pub fn watch() {
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    let current_dir = env::current_dir().expect("Failed to determine current directory");

    inotify
        .add_watch(
            "/home/jferry/projects/desktopd/tests/test1",
            WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE,
        )
        .expect("Failed to add inotify watch");

    inotify
        .add_watch(
            "/home/jferry/projects/desktopd/tests/test2",
            WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE,
        )
        .expect("Failed to add inotify watch");

    println!("Watching current directory for activity...");

    let mut buffer = [0u8; 4096];
    loop {
        inotify
            .add_watch(
                "/home/jferry/projects/desktopd/tests/test3",
                WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE,
            )
            .expect("Failed to add inotify watch");
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        for event in events {
            if event.mask.contains(EventMask::CREATE) && !event.mask.contains(EventMask::ISDIR) {
                println!("File created: {:?} => {:?}", event.wd, event.name);
            } else if event.mask.contains(EventMask::DELETE) {
                if event.mask.contains(EventMask::ISDIR) {
                    println!("Directory deleted: {:?}", event.name);
                } else {
                    println!("File deleted: {:?}", event.name);
                }
            } else if event.mask.contains(EventMask::MODIFY) {
                if event.mask.contains(EventMask::ISDIR) {
                    println!("Directory modified: {:?}", event.name);
                } else {
                    println!("File modified: {:?}", event.name);
                }
            }
        }
    }
}
