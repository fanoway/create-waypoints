# Template: create_waypoints
Tool to convert assignments in GeoJSON files to waypoints in a new gpx file.

Deisgned for use in converfting SAR TOPO assignments to waypoints/markers as corners

GeoJson translation to GPX is done on the client side 

## Usage
GeoJson can be exported from SAR topo containing the assignment

Only do 1 assignment at a time(for now)

Returns a GPX file with the corner points labelled corner 0 -> corner n-1

Points of interest / waypoints seperate to the assignment polygon should be exported directly as GPX

