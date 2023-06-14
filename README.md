# PEOG

PEOG written in Rust, python code generator that checks if input is odd or even. Code example from it looks like this:

```python
[...]
elif input == 32:
	print("Even")
elif input == 33:
	print("Odd")
elif input == 34:
	print("Even")
[...]
```
Isn't it BEAUTIFUL???


## GETTING STARTED

- Clone the repository

    ```bash
    mkdir peog
    cd peog
    git clone https://github.com/kmtrebacz/peog.git .
    ```

- Compile the code

    ```bash
    cargo build
    ```

- Run the program

    ```bash
    ./target/debug/peog even_odd_python -20000 20000
    ```

- If you did't get any errors, you can run this BEAUTIFUL peace of code

    ```bash
    python ./even_odd_python.py
    ```

- Enjoy!
