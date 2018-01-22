// parsing geojson example from:
// https://github.com/urschrei/geojson_example/blob/master/src/owned.rs
extern crate geojson;
use self::geojson::{GeoJson, Geometry, Value};

extern crate rayon;
use rayon::prelude::*;

/// Process GeoJSON geometries
fn match_geometry(geom: &Geometry) {
    match geom.value {
        Value::Polygon(_) => println!("Matched a Polygon"),
        Value::MultiPolygon(_) => println!("Matched a MultiPolygon"),
        Value::GeometryCollection(ref collection) => {
            println!("Matched a GeometryCollection");
            // GeometryCollections contain other Geometry types, and can nest
            // we deal with this by recursively processing each geometry
            collection
                .par_iter()
                .for_each(|geometry| match_geometry(geometry))
        }
        // Point, LineString, and their Multi– counterparts
        _ => println!("Matched some other geometry"),
    }
}

/// Process top-level GeoJSON items
pub fn process_geojson(gj: &GeoJson) {
    match *gj {
        GeoJson::FeatureCollection(ref collection) => collection.features
            // Iterate in parallel when appropriate
            .par_iter()
            // Only pass on non-empty geometries, doing so by reference
            .filter_map(|feature| feature.geometry.as_ref())
            .for_each(|geometry| match_geometry(geometry)),
        GeoJson::Feature(ref feature) => {
            if let Some(ref geometry) = feature.geometry {
                match_geometry(geometry)
            }
        }
        GeoJson::Geometry(ref geometry) => match_geometry(geometry),
    }
}

