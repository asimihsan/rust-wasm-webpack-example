init:
	(cd web && npm i)

build:
	./build.sh

serve:
	(cd web && npm run start)