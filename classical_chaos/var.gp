set style data points
set datafile separator ","
set terminal png size 1000,1000
set output "var.png"
# set xrange [-0.05:0.5]
# set yrange [-0.45:0.45]
set xlabel "x"
set ylabel "y"
set key off
set pointsize 0.1

plot "var.csv" with line