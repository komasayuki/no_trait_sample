use once_cell::sync::Lazy;

trait Animal {
    async fn meow(&self);
}

cfg_if::cfg_if! {
    if #[cfg(feature = "cat")] {
        
        static ANIMAL: Lazy<Cat> = Lazy::new(|| {
            Cat{}
        });

        struct Cat {}
        impl Animal for Cat {
            async fn meow(&self) {
                println!("Cat meow");
            }
        }
    }
}


cfg_if::cfg_if! {
    if #[cfg(feature = "dog")] {
        
        static ANIMAL: Lazy<Dog> = Lazy::new(|| {
            Dog{}
        });

        struct Dog {}
        impl Animal for Dog {
            async fn meow(&self) {
                println!("Dog meow");
            }
        }
    }
}






#[tokio::main]
async fn main() {

    println!("Hello, world!");
    ANIMAL.meow().await;
}
