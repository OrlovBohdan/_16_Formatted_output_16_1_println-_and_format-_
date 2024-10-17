#[test]

/*

fn main() {
    let s1 = "hello";
    /* Fill in the blank */
    let s = format!(__);
    assert_eq!(s, "hello, world!");
}
*/

fn main() {
    let s1 = "hello";
    /* Fill in the blank */
    let s = format!("{}, world!", s1);
    assert_eq!(s, "hello, world!");
}


/*
format!: Це макрос, який створює форматований рядок. Він працює подібно до println!, але не виводить рядок на консоль, а повертає його у вигляді значення.
{}: Це плейсхолдер для значення, яке ви хочете вставити у рядок. У даному випадку, ми вставляємо s1.
s1: Це змінна, що містить рядок "hello"
*/