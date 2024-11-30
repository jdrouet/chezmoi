build-devenv:
    docker build -t jdrouet/chezmoi:devenv -f dev.Dockerfile .

run-devenv: build-devenv
    docker run -it --rm -v $(pwd):/code -w /code jdrouet/chezmoi:devenv /bin/bash
