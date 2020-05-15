.DEFAULT_GOAL:=help
.PHONY: help rust-raw actix go-net go-gin gunicorn-flask

PORT_OPTIONS=_8080='rust-raw', _8088='actix:', _9000='go-net', _9001='go-gin', _5000='gunicorn-flask'

help: ## Show all the available make commands
	@echo "\n========================================================================================================================================================================================"
	@awk '/```ascii/{a=1; next}/```/{a=0}(a==1){print}' README.md
	@echo "=========================================================================================================================================================================================\n"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

rust-raw: ## run raw rust http
	rustc rust-tcp-server.rs
	./rust-tcp-server

rocket:
	@echo "you probably just wanna run metrix..."

actix: ## run actix-web http server
	cargo run # --release # if you wanna run optimized build

go-net: ## raw go net/http server
	go run http.go

go-gin: ## run go gin server
	go get -u github.com/gin-gonic/gin
	go run gin.go

gunicorn-flask: ## run simple flask/gunicorn server
	cd flask && virtualenv venv -p python
	flask/venv/bin/pip install flask gunicorn
	cd flask && venv/bin/gunicorn -c gunicorn.conf.py app:app

run-benchmark: ## run wrk for benchmarking
ifeq ($(PORT),)
	@echo "please set PORT, example: \`make run-benchmark PORT=5000\`"
	@echo "Options: "$(PORT_OPTIONS)
else
	@wrk -t100 -c100 -d1s http://127.0.0.1:$(PORT)/ping || echo "please install wrk: https://github.com/wg/wrk"
	@python -c "print 'Results for port $(PORT): ' + dict($(PORT_OPTIONS)).get('_' + '$(PORT)', '')"
endif
