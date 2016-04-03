var gulp       = require('gulp');
var browserify = require('gulp-browserify');
var vueify     = require('vueify');
var stylus     = require('gulp-stylus');

gulp.task('scripts', function() {
    gulp.src('src/js/main.js')
        .pipe(browserify({
            transform: 'vueify'
        }))
        .pipe(gulp.dest('./public/js'));
});

gulp.task('stylus', function() {
    gulp.src('src/stylus/app.styl')
        .pipe(stylus({
            paths:  ['node_modules'],
            import: ['jeet/stylus/jeet'],
            'include css': true
        }))
        .pipe(gulp.dest('./public/css'));
});

gulp.task('watch', function() {
    gulp.watch(['src/js/**/*.{js,vue}'], ['scripts'])
    gulp.watch(['src/stylus/**/*.styl'], ['stylus'])
});

gulp.task('default', ['scripts', 'stylus', 'watch']);
