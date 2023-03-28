set terminal pngcairo
set output 'plots/well_energies.png'

set xlabel "Energy of the well"
set ylabel "Frequency"
set grid
set style fill solid 1 noborder

bin_width = 10000
bin_number(x) = floor(x/bin_width)
rounded(x) = bin_width * (bin_number(x) + 0.5)
set boxwidth bin_width
# set xrange [0:1e6]
# set xtics ("0" 0, "2·10^5" 2e5, "4·10^5" 4e5, "6·10^5" 6e5, "8·10^5" 8e5,  "1·10^6" 1e6)
plot 'generated_data/well_energies.txt' using (rounded($1)):(1.0) smooth freq with boxes notitle
