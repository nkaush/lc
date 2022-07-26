main.%:
	cp ../templates/$@ ./$@

rust: 
	rustc main.rs && ./main

python:
	python3 main.py

cpp: 
	clang++ -std=c++17 main.cpp -o main && ./main

c: 
	gcc main.c -o main && ./main

clean:
	rm -f main