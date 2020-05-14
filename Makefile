.DEFAULT_GOAL:=help
.PHONY: help rust-raw actix go-net go-gin gunicorn-flask

help: ## Show all the available make commands
	@echo "\n======================================================================================================================================================================="
	@awk '/```ascii/{a=1; next}/```/{a=0}(a==1){print}' README.md
	@echo "=======================================================================================================================================================================\n"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

rust-raw: ## raw rust http | port 8080
	rustc rust-tcp-server.rs
	./rust-tcp-server

rocket:
	@echo "you probably just wanna run metrix..."

actix: ## run actix-web http server | port 8088
	cargo run # --release # if you wanna run optimized build

go-net: ## raw go net/http server | port 9000
	go run http.go

go-gin: ## run go gin server | port 9001
	go get -u github.com/gin-gonic/gin
	go run gin.go

gunicorn-flask: ## simple flask/gunicorn server | port 5000
	cd flask && virtualenv venv -p python
	flask/venv/bin/pip install flask gunicorn
	cd flask && venv/bin/gunicorn -c gunicorn.conf.py app:app

run-benchmark: ## run wrk
ifeq ($(PORT),)
	@echo "please set PORT, example: \`make run-benchmark PORT=5000\`"
else
	@wrk -t100 -c100 -d1s http://127.0.0.1:$(PORT)/ping || echo "please install wrk: https://github.com/wg/wrk"
endif
