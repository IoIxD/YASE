all:
	GODEBUG=cgocheck=0 GOOS=js GOARCH=wasm go build -o web/yase.wasm . 

run:
	GODEBUG=cgocheck=0 GOOS=js GOARCH=wasm go build -o web/yase.wasm . 
	python3 -m http.server --directory web/ 8081
