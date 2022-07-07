.PHONY : all wasm x86 run

all: wasm

run: wasm-run

x86:
	go build .

wasm: 
	GODEBUG=cgocheck=0 GOOS=js GOARCH=wasm go build -o web/yase.wasm . 

x86-run: 
	go run .

wasm-run:
	make wasm
	python3 -m http.server --directory web/ 8081
