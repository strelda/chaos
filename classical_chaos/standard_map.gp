set style data points
set datafile separator ","
set terminal png size 1000,1000
set output image_name
# set xrange [0:1]
# set yrange [0:1]
set xlabel "x"
set ylabel "y"
set key off
set pointsize 0.1
unset colorbox


plot name using 1:2:(int($0/1000)) with points pointtype 7 linecolor palette
