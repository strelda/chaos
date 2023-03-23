set style data points
set datafile separator ","
set terminal png size 1000,1000
set output "deviation_vector.png"
# set xrange [-0.05:0.5]
# set yrange [-0.45:0.45]
set xlabel "t"
set ylabel "log(mLCE)"
set key on
set pointsize 0.1
# log scale
set logscale x
# set logscale y



plot "deviation_vector_k1.csv" with line title '1', \
     "deviation_vector_k0.1.csv" with line title '2', \
     "deviation_vector_k0.01.csv" with line title '3'