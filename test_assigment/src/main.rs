use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();
    table.insert("test 1".to_string(), vec!["test_1_work_1".to_string(), "atest_1_work_2".to_string()]);
    table.insert("test 2".to_string(), vec!["test_2_work_1".to_string(), "atest_2_work_2".to_string()]);
    table.insert("test 3".to_string(), vec!["test_3_work_1".to_string(), "atest_3_work_2".to_string()]);
    show(&table);
    sort_works(&mut table);
    show(&table);
    assert_eq!(table["test 1"][1], "test_1_work_1");

    let x = 10;
    let y = 20;
    let mut test = &x;
    println!("test = {}",*test);
    test = &y;
    println!("test again = {}", *test);
}

fn show(table: &Table){
    for (artist, works) in table{
        println!("Works by : {}", artist);
        for work in works{
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table){
    for (_artist, works) in table{
        works.sort()
    }
}
