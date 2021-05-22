NAME=rusgba
VERSION=latest
CWD=`pwd`

build:
	docker build -t $(NAME):$(VERSION) .

run:
	docker run -it --rm \
		-p 8080:8080 \
        -v $(CWD)/../src:/home/rusgba/src \
        -v $(CWD)/../tests:/home/rusgba/tests \
        -v $(CWD)/../Cargo.toml:/home/rusgba/Cargo.toml \
        -v $(CWD)/../Makefile:/home/rusgba/Makefile \
        -v $(CWD)/../app:/home/rusgba/app \
        -v $(CWD)/../scripts:/home/rusgba/scripts \
		--name $(NAME) \
		$(NAME):$(VERSION) bash

exec:
	docker exec -it $(NAME) bash

stop:
	docker rm -f $(NAME)

contener=`docker ps -a -q`
image=`docker images | awk '/^<none>/ { print $$3 }'`

clean:
	@if [ "$(image)" != "" ] ; then \
		docker rmi $(image); \
		fi
	@if [ "$(contener)" != "" ] ; then \
		docker rm $(contener); \
		fi
