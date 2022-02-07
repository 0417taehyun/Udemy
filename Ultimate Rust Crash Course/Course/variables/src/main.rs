// 상수이기 때문에 main() 함수 바깥에 정의해서 사용 가능
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let missiles = 8;
    let ready = 2;

    println!("Firing {} of my {} missiles...", ready, missiles);

    /*
    Rust는 기본적으로 Immutable Variables다.
    For what? Safety, Concurrency, Speed

    Safety: Bugs can't happen if a value never changes
    Concurrency: Data that never changes can be shared between multiple threads without locks
    Speed: Compiler can also do extra optimizations on data it knows won't change
    */

    // missiles 및 ready는 immutable이기 때문에 아래 코드는 오류가 발생한다.
    // missiles = missiles - ready; > Cannot assign twice to immutable variables
    // println!("{} missiles left", missiles);

    // 아래와 같이 mut 키워드를 활용하여 값을 변화시킬 수 있다.
    let mut new_missiles = 8;
    let new_ready = 2;

    println!("This is new! Firing {} of my {} missiles", new_ready, new_missiles);

    new_missiles -= new_ready;
    println!("{} missiles left", new_missiles);

    // 아래와 같이 let이 아닌 const 키워드를 사용하면 mut 키워드 자체의 사용을 막아 불변셩 그 자체를 의미한다.
    // 이때 유의할 점은 아래 f32처럼 데이터 자료형을 선언해야 한다는 것이다.
    const NEW_PI: f32 = 3.14;
    // NEW_PI = 10; > Invalid left-hand side
    println!("This is PI: {}", NEW_PI);

    let another_missiles = STARTING_MISSILES;
    let another_ready = READY_AMOUNT;
    println!("With Constant, Firing {} of my {} missiles", another_ready, another_missiles);

    let unused = "unused"; // > unused variable `unused`

    println!("Directly {}", STARTING_MISSILES - READY_AMOUNT);

    // 아래와 같이 여러 변수를 한 줄에 정의할 수 있는데 각각의 바인딩마다 mut 키워드를 작성해줘야 한다.
    let (mut a, mut b) = ("A", "B");
    println!("This is {}, {}", a, b);

    a = "New A";
    b = "New B";
    println!("This is New! {}. {}", a, b);
}
