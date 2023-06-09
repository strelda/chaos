set terminal pngcairo
set output 'plots/2d/endif.png'

set xlabel "S"
set ylabel "Frequency"
set grid
set style fill solid 1 noborder

bin_width = 100
bin_number(x) = floor(x/bin_width)
rounded(x) = bin_width * (bin_number(x) + 0.5)
set boxwidth bin_width
# set logscale x
# set xtics ("0" 0, "2·10^3" 2e3, "4·10^3" 4e3, "6·10^3" 6e3, "8·10^3" 8e3, "1·10^4" 1e4, "1.2·10^3" 1.2*1e4, "1.4·10^4" 1.4*1e4)

plot 'endif.txt' using (rounded($1)):(1.0) smooth freq with boxes notitle
