# MLCS

A Crate for finding the MLCS of an arbitrarily big number of strings

## Testing

1. **Clone the repository:**

    ```sh
    git clone https://github.com/epita-rs/MLCS.git
    cd MLCS
    ```

2. **Start the tests**

    ```sh
    cargo test --release
    ```
3. **Start the performance benchmark**

    ```sh
    cargo bench
    ```
## Remarks
You can expect this function to be very fast when the strings do not have
much letters in common. Complexity tends to grow quickly as the number of strings
grow. The size of the strings is usually not the issue. 

Moreover having a small string in the input will tremendously improve the performance.
## Documentation
Interesting ressources concerning mlcs : https://github.com/lmcarreiro/multiple-lcs?tab=readme-ov-file.
