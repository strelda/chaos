using CSV, DataFrames, Distributions, LinearAlgebra, Plots

# global constants
const T = 1000
const K = 1.0
const π = pi

println("T = $T, K = $K")

function standard_map(x0::Float64, y0::Float64, k::Float64)
    x = zeros(T+1)
    y = zeros(T+1)
    x[1] = x0
    y[1] = y0
    
    # evolve
    for i in 1:T
        y[i+1] = y[i] - (k/(2*π))*sin(2*π*x[i])
        x[i+1] = x[i] + y[i+1]
    end
    
    # modulo 1
    x .-= floor.(x) .+ 0.5
    y .-= 0.0
    
    [x, y]
end

function save_standard_map(filename::AbstractString, x0_all::LinSpace, y0_all::LinSpace)
    file = CSV.File(filename, "w")
    time = now()
    
    for y0 in y0_all, x0 in x0_all
        [x, y] = standard_map(x0, y0, K)
        CSV.write(file, DataFrame(x=x, y=y))
    end
    
    println("standard map written, time elapsed: $(now() - time) ms")
end

x0_all = range(-0.5, 0.5, length=1)
y0_all = range(-0.45, 0.45, length=100)
save_standard_map("standard_map.csv", x0_all, y0_all)

# standard_map(0.0, 0.0, K)
