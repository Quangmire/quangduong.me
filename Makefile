run:
	./serve -k

compile:
	browserify src/index.jsx -o static/index.js -t [ babelify --presets [ env react ] ]

minify:
	browserify src/index.jsx -t [ babelify --presets [ env react ] ] | uglifyjs -o static/index.js
