using CSV
using DataFrames
using Dates
using GLMakie
using PlotUtils: optimize_ticks

df = CSV.read("test.csv", DataFrame)

df = filter(df -> 50 >= df.MSW >= 30, df)

df.ts = unix2datetime.(df.UnixTime) + Hour(11)
ts_ticks = optimize_ticks(df.ts[1], df.ts[end])[1]

fig = Figure()
menu = Menu(fig, options = ["viridis", "heat", "blues"])
fig[1, 1] = vgrid!(Label(fig, "Colormap", width=180), menu; tellheight = false, width = 200)
ax = Axis(fig[1, 2])
scat = scatter!(datetime2rata.(df.ts), df.MSW, color = df.TimeStable, 
                            markersize=5, colormap=:thermal)

cb = Colorbar(fig[1, 3], scat)

on(menu.selection) do s
    scat.colormap = s
end

# Formatting 
ax.xticks[] = (datetime2rata.(ts_ticks), Dates.format.(ts_ticks, "dd/mm/yy"))
ax.title = "A plot"
ax.xlabel = "Time (dd/mm/yy)"
ax.ylabel = "Y"

fig