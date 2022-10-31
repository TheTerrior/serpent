use ncurses as nc;

fn main() {
    nc::initscr();
    nc::noecho();


    loop {
        let x = nc::getch();
        nc::clear();
        nc::addstr(&x.to_string());
        nc::refresh();

        if x == 'q' as i32 {
            break;
        }
    }

    nc::endwin();
}