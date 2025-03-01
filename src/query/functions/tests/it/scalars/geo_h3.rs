// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::Write;

use databend_common_expression::types::*;
use databend_common_expression::FromData;
use goldenfile::Mint;

use super::run_ast;

#[test]
fn test_geo_h3() {
    let mut mint = Mint::new("tests/it/scalars/testdata");
    let file = &mut mint.new_goldenfile("geo_h3.txt").unwrap();

    test_h3_to_geo(file);
    test_h3_to_geo_boundary(file);
    test_h3_k_ring(file);
    test_h3_is_valid(file);
    test_h3_get_resolution(file);
    test_h3_edge_length_m(file);
    test_h3_edge_length_km(file);
    test_h3_get_base_cell(file);
    test_h3_hex_area_m2(file);
    test_h3_hex_area_km2(file);
    test_h3_indexes_are_neighbors(file);
    test_h3_to_children(file);
    test_h3_to_parent(file);
    test_h3_to_string(file);
    test_string_to_h3(file);
    test_h3_is_res_class_iii(file);
    test_h3_is_pentagon(file);
    test_h3_get_faces(file);
    test_h3_cell_area_m2(file);
    test_h3_cell_area_rads2(file);
    test_h3_to_center_child(file);
    test_h3_exact_edge_length_m(file);
    test_h3_exact_edge_length_km(file);
    test_h3_exact_edge_length_rads(file);
    test_h3_num_hexagons(file);
    test_h3_line(file);
    test_h3_distance(file);
    test_h3_hex_ring(file);
    test_h3_get_unidirectional_edge(file);
    test_h3_unidirectional_edge_is_valid(file);
    test_h3_get_origin_index_from_unidirectional_edge(file);
    test_h3_get_destination_index_from_unidirectional_edge(file);
    test_h3_get_indexes_from_unidirectional_edge(file);
    test_h3_get_unidirectional_edges_from_hexagon(file);
    test_h3_get_unidirectional_edge_boundary(file);
    test_h3_edge_angle(file);
}

fn test_h3_to_geo(file: &mut impl Write) {
    run_ast(file, "h3_to_geo(-1)", &[]);
    run_ast(file, "h3_to_geo(0)", &[]);
    run_ast(file, "h3_to_geo(1)", &[]);

    run_ast(file, "h3_to_geo(644325524701193974)", &[]);

    run_ast(file, "h3_to_geo(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![
            644325529094369568,
            644325528627451570,
            644325528491955313,
        ]),
    )]);
}

fn test_h3_to_geo_boundary(file: &mut impl Write) {
    run_ast(file, "h3_to_geo_boundary(-1)", &[]);
    run_ast(file, "h3_to_geo_boundary(0)", &[]);
    run_ast(file, "h3_to_geo_boundary(1)", &[]);

    run_ast(file, "h3_to_geo_boundary(644325524701193974)", &[]);

    run_ast(file, "h3_to_geo_boundary(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![
            644325524701193974,
            644325529094369568,
            644325528627451570,
            644325528491955313,
        ]),
    )]);
}

fn test_h3_k_ring(file: &mut impl Write) {
    run_ast(file, "h3_k_ring(-1, 1)", &[]);
    run_ast(file, "h3_k_ring(0, 0)", &[]);
    run_ast(file, "h3_k_ring(0, -1)", &[]);

    run_ast(file, "h3_k_ring(644325524701193974, -1)", &[]);
    run_ast(file, "h3_k_ring(644325524701193974, 0)", &[]);

    run_ast(file, "h3_k_ring(644325524701193974, 1)", &[]);
    run_ast(file, "h3_k_ring(644325524701193974, 2)", &[]);
    run_ast(file, "h3_k_ring(644325524701193974, 3)", &[]);

    run_ast(file, "h3_k_ring(h3, k)", &[
        (
            "h3",
            UInt64Type::from_data(vec![
                644325524701193974,
                644325529094369568,
                644325528627451570,
                644325528491955313,
            ]),
        ),
        ("k", UInt32Type::from_data(vec![1, 2, 3, 4])),
    ]);
}

fn test_h3_is_valid(file: &mut impl Write) {
    run_ast(file, "h3_is_valid(0)", &[]);
    run_ast(file, "h3_is_valid(644325524701193974)", &[]);

    run_ast(file, "h3_is_valid(res)", &[(
        "res",
        UInt64Type::from_data(vec![
            1,
            644325524701193974,
            644325529094369568,
            644325528627451570,
            644325528491955313,
        ]),
    )]);
}

fn test_h3_get_resolution(file: &mut impl Write) {
    run_ast(file, "h3_get_resolution(0)", &[]);
    run_ast(file, "h3_get_resolution(644325524701193974)", &[]);

    run_ast(file, "h3_get_resolution(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![
            644325524701193974,
            644325529094369568,
            644325528627451570,
            644325528491955313,
        ]),
    )]);
}

fn test_h3_edge_length_m(file: &mut impl Write) {
    run_ast(file, "h3_edge_length_m(0)", &[]);
    run_ast(file, "h3_edge_length_m(1)", &[]);
    run_ast(file, "h3_edge_length_m(15)", &[]);
    run_ast(file, "h3_edge_length_m(16)", &[]);

    run_ast(file, "h3_edge_length_m(res)", &[(
        "res",
        UInt8Type::from_data(vec![1, 2, 3, 4]),
    )]);
}

fn test_h3_edge_length_km(file: &mut impl Write) {
    run_ast(file, "h3_edge_length_km(0)", &[]);
    run_ast(file, "h3_edge_length_km(1)", &[]);
    run_ast(file, "h3_edge_length_km(15)", &[]);
    run_ast(file, "h3_edge_length_km(16)", &[]);

    run_ast(file, "h3_edge_length_km(res)", &[(
        "res",
        UInt8Type::from_data(vec![1, 2, 3, 4]),
    )]);
}

fn test_h3_get_base_cell(file: &mut impl Write) {
    run_ast(file, "h3_get_base_cell(0)", &[]);
    run_ast(file, "h3_get_base_cell(644325524701193974)", &[]);

    run_ast(file, "h3_get_base_cell(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![
            644325524701193974,
            644325529094369568,
            644325528627451570,
            644325528491955313,
        ]),
    )]);
}

fn test_h3_hex_area_m2(file: &mut impl Write) {
    run_ast(file, "h3_hex_area_m2(0)", &[]);
    run_ast(file, "h3_hex_area_m2(1)", &[]);
    run_ast(file, "h3_hex_area_m2(15)", &[]);
    run_ast(file, "h3_hex_area_m2(16)", &[]);

    run_ast(file, "h3_hex_area_m2(res)", &[(
        "res",
        UInt8Type::from_data(vec![1, 2, 3, 4]),
    )]);
}

fn test_h3_hex_area_km2(file: &mut impl Write) {
    run_ast(file, "h3_hex_area_km2(0)", &[]);
    run_ast(file, "h3_hex_area_km2(1)", &[]);
    run_ast(file, "h3_hex_area_km2(15)", &[]);
    run_ast(file, "h3_hex_area_km2(16)", &[]);

    run_ast(file, "h3_hex_area_km2(res)", &[(
        "res",
        UInt8Type::from_data(vec![1, 2, 3, 4]),
    )]);
}

fn test_h3_indexes_are_neighbors(file: &mut impl Write) {
    run_ast(file, "h3_indexes_are_neighbors(0, 0)", &[]);
    run_ast(
        file,
        "h3_indexes_are_neighbors(644325524701193974, 644325524701193897)",
        &[],
    );
    run_ast(
        file,
        "h3_indexes_are_neighbors(644325524701193974, 644325529094369568)",
        &[],
    );

    run_ast(file, "h3_indexes_are_neighbors(h3, a_h3)", &[
        (
            "h3",
            UInt64Type::from_data(vec![
                644325524701193974,
                644325524701193974,
                644325524701193974,
            ]),
        ),
        (
            "a_h3",
            UInt64Type::from_data(vec![
                644325524701193897,
                644325524701193899,
                644325528627451570,
            ]),
        ),
    ]);
}

fn test_h3_to_children(file: &mut impl Write) {
    run_ast(file, "h3_to_children(0, 1)", &[]);
    run_ast(file, "h3_to_children(644325524701193897, 15)", &[]);
    run_ast(file, "h3_to_children(644325524701193974, 16)", &[]);

    run_ast(file, "h3_to_children(h3, res)", &[
        (
            "h3",
            UInt64Type::from_data(vec![
                635318325446452991,
                635318325446452991,
                635318325446452991,
            ]),
        ),
        ("res", UInt8Type::from_data(vec![13, 14, 15])),
    ]);
}

fn test_h3_to_parent(file: &mut impl Write) {
    run_ast(file, "h3_to_parent(0, 1)", &[]);
    run_ast(file, "h3_to_parent(635318325446452991, 16)", &[]);
    run_ast(file, "h3_to_parent(635318325446452991, 14)", &[]);
    run_ast(file, "h3_to_parent(635318325446452991, 12)", &[]);

    run_ast(file, "h3_to_parent(h3, res)", &[
        (
            "h3",
            UInt64Type::from_data(vec![
                635318325446452991,
                635318325446452991,
                635318325446452991,
            ]),
        ),
        ("res", UInt8Type::from_data(vec![10, 12, 15])),
    ]);
}

fn test_h3_to_string(file: &mut impl Write) {
    run_ast(file, "h3_to_string(0)", &[]);
    run_ast(file, "h3_to_string(635318325446452991)", &[]);

    run_ast(file, "h3_to_string(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![
            635318325446452991,
            644325524701193897,
            599686042433355775,
        ]),
    )]);
}

fn test_string_to_h3(file: &mut impl Write) {
    run_ast(file, "string_to_h3('')", &[]);
    run_ast(file, "string_to_h3('xxxx')", &[]);
    run_ast(file, "h3_to_string('8d11aa6a38826ff')", &[]);

    run_ast(file, "string_to_h3(h3_str)", &[(
        "h3_str",
        StringType::from_data(vec![
            "8d11aa6a38826ff",
            "8f11aa6a38826a9",
            "85283473fffffff",
        ]),
    )]);
}

fn test_h3_is_res_class_iii(file: &mut impl Write) {
    run_ast(file, "h3_is_res_class_iii(0)", &[]);
    run_ast(file, "h3_is_res_class_iii(635318325446452991)", &[]);

    run_ast(file, "h3_is_res_class_iii(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![
            635318325446452991,
            644325524701193897,
            599686042433355775,
        ]),
    )]);
}

fn test_h3_is_pentagon(file: &mut impl Write) {
    run_ast(file, "h3_is_pentagon(0)", &[]);
    run_ast(file, "h3_is_pentagon(599119489002373119)", &[]);

    run_ast(file, "h3_is_pentagon(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![
            599119489002373119,
            644325524701193897,
            599686042433355775,
        ]),
    )]);
}

fn test_h3_get_faces(file: &mut impl Write) {
    run_ast(file, "h3_get_faces(0)", &[]);
    run_ast(file, "h3_get_faces(599119489002373119)", &[]);

    run_ast(file, "h3_get_faces(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![
            599119489002373119,
            599686042433355775,
            599686042433355775,
        ]),
    )]);
}

fn test_h3_cell_area_m2(file: &mut impl Write) {
    run_ast(file, "h3_cell_area_m2(0)", &[]);
    run_ast(file, "h3_cell_area_m2(599119489002373119)", &[]);

    run_ast(file, "h3_cell_area_m2(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![
            599119489002373119,
            599686042433355775,
            599686042433355775,
        ]),
    )]);
}

fn test_h3_cell_area_rads2(file: &mut impl Write) {
    run_ast(file, "h3_cell_area_rads2(0)", &[]);
    run_ast(file, "h3_cell_area_rads2(599119489002373119)", &[]);

    run_ast(file, "h3_cell_area_rads2(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![
            599119489002373119,
            599686042433355775,
            599686042433355775,
        ]),
    )]);
}

fn test_h3_to_center_child(file: &mut impl Write) {
    run_ast(file, "h3_to_center_child(0, 1)", &[]);
    run_ast(file, "h3_to_center_child(599119489002373119, 16)", &[]);
    run_ast(file, "h3_to_center_child(599119489002373119, 15)", &[]);

    run_ast(file, "h3_to_center_child(h3, res)", &[
        (
            "h3",
            UInt64Type::from_data(vec![599119489002373119, 635318325446452991]),
        ),
        ("res", UInt8Type::from_data(vec![14, 15])),
    ]);
}

fn test_h3_exact_edge_length_m(file: &mut impl Write) {
    run_ast(file, "h3_exact_edge_length_m(0)", &[]);
    run_ast(file, "h3_exact_edge_length_m(599119489002373119)", &[]);
    run_ast(file, "h3_exact_edge_length_m(1319695429381652479)", &[]);

    run_ast(file, "h3_exact_edge_length_m(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![1319695429381652479, 1391753023419580415]),
    )]);
}

fn test_h3_exact_edge_length_km(file: &mut impl Write) {
    run_ast(file, "h3_exact_edge_length_km(0)", &[]);
    run_ast(file, "h3_exact_edge_length_km(599119489002373119)", &[]);
    run_ast(file, "h3_exact_edge_length_km(1319695429381652479)", &[]);

    run_ast(file, "h3_exact_edge_length_km(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![1319695429381652479, 1391753023419580415]),
    )]);
}

fn test_h3_exact_edge_length_rads(file: &mut impl Write) {
    run_ast(file, "h3_exact_edge_length_rads(0)", &[]);
    run_ast(file, "h3_exact_edge_length_rads(599119489002373119)", &[]);
    run_ast(file, "h3_exact_edge_length_rads(1319695429381652479)", &[]);

    run_ast(file, "h3_exact_edge_length_rads(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![1319695429381652479, 1391753023419580415]),
    )]);
}

fn test_h3_num_hexagons(file: &mut impl Write) {
    run_ast(file, "h3_num_hexagons(0)", &[]);
    run_ast(file, "h3_num_hexagons(16)", &[]);
    run_ast(file, "h3_num_hexagons(10)", &[]);

    run_ast(file, "h3_num_hexagons(res)", &[(
        "res",
        UInt8Type::from_data(vec![10, 15]),
    )]);
}

fn test_h3_line(file: &mut impl Write) {
    run_ast(file, "h3_line(0, 0)", &[]);
    run_ast(file, "h3_line(599119489002373119, 0)", &[]);
    run_ast(file, "h3_line(599119489002373119, 599119491149856767)", &[]);

    run_ast(file, "h3_line(h3, a_h3)", &[
        (
            "h3",
            UInt64Type::from_data(vec![599119489002373119, 599119489002373119]),
        ),
        (
            "a_h3",
            UInt64Type::from_data(vec![599119491149856767, 599119492223598591]),
        ),
    ]);
}

fn test_h3_distance(file: &mut impl Write) {
    run_ast(file, "h3_distance(0, 0)", &[]);
    run_ast(file, "h3_distance(599119489002373119, 0)", &[]);
    run_ast(
        file,
        "h3_distance(599119489002373119, 599119491149856767)",
        &[],
    );

    run_ast(file, "h3_distance(h3, a_h3)", &[
        (
            "h3",
            UInt64Type::from_data(vec![599119489002373119, 599119489002373119]),
        ),
        (
            "a_h3",
            UInt64Type::from_data(vec![599119491149856767, 599119492223598591]),
        ),
    ]);
}

fn test_h3_hex_ring(file: &mut impl Write) {
    run_ast(file, "h3_hex_ring(0, 0)", &[]);
    run_ast(file, "h3_hex_ring(599686042433355775, 0)", &[]);
    run_ast(file, "h3_hex_ring(599119489002373119, 2)", &[]);
    run_ast(file, "h3_hex_ring(599686042433355775, 2)", &[]);

    run_ast(file, "h3_distance(h3, k)", &[
        (
            "h3",
            UInt64Type::from_data(vec![599686042433355775, 644325524701193897]),
        ),
        ("k", UInt32Type::from_data(vec![2, 3])),
    ]);
}

fn test_h3_get_unidirectional_edge(file: &mut impl Write) {
    run_ast(file, "h3_get_unidirectional_edge(0, 0)", &[]);
    run_ast(
        file,
        "h3_get_unidirectional_edge(644325524701193897, 0)",
        &[],
    );
    run_ast(
        file,
        "h3_get_unidirectional_edge(644325524701193897, 644325524701193754)",
        &[],
    );

    run_ast(file, "h3_get_unidirectional_edge(h3, a_h3)", &[
        (
            "h3",
            UInt64Type::from_data(vec![644325524701193897, 644325524701193897]),
        ),
        (
            "a_h3",
            UInt64Type::from_data(vec![644325524701193754, 644325524701193901]),
        ),
    ]);
}

fn test_h3_unidirectional_edge_is_valid(file: &mut impl Write) {
    run_ast(file, "h3_unidirectional_edge_is_valid(0)", &[]);
    run_ast(
        file,
        "h3_unidirectional_edge_is_valid(1248204388774707199)",
        &[],
    );

    run_ast(file, "h3_unidirectional_edge_is_valid(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![1248204388774707199, 644325524701193754]),
    )]);
}

fn test_h3_get_origin_index_from_unidirectional_edge(file: &mut impl Write) {
    run_ast(file, "h3_get_origin_index_from_unidirectional_edge(0)", &[]);
    run_ast(
        file,
        "h3_get_origin_index_from_unidirectional_edge(1248204388774707199)",
        &[],
    );

    run_ast(file, "h3_get_origin_index_from_unidirectional_edge(h3)", &[
        (
            "h3",
            UInt64Type::from_data(vec![1248204388774707199, 1319695429381652479]),
        ),
    ]);
}

fn test_h3_get_destination_index_from_unidirectional_edge(file: &mut impl Write) {
    run_ast(
        file,
        "h3_get_destination_index_from_unidirectional_edge(0)",
        &[],
    );
    run_ast(
        file,
        "h3_get_destination_index_from_unidirectional_edge(1248204388774707199)",
        &[],
    );

    run_ast(
        file,
        "h3_get_destination_index_from_unidirectional_edge(h3)",
        &[(
            "h3",
            UInt64Type::from_data(vec![1248204388774707199, 1319695429381652479]),
        )],
    );
}

fn test_h3_get_indexes_from_unidirectional_edge(file: &mut impl Write) {
    run_ast(file, "h3_get_indexes_from_unidirectional_edge(0)", &[]);
    run_ast(
        file,
        "h3_get_indexes_from_unidirectional_edge(1248204388774707199)",
        &[],
    );

    run_ast(file, "h3_get_indexes_from_unidirectional_edge(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![1248204388774707199, 1319695429381652479]),
    )]);
}

fn test_h3_get_unidirectional_edges_from_hexagon(file: &mut impl Write) {
    run_ast(file, "h3_get_unidirectional_edges_from_hexagon(0)", &[]);
    run_ast(
        file,
        "h3_get_unidirectional_edges_from_hexagon(644325524701193754)",
        &[],
    );

    run_ast(file, "h3_get_unidirectional_edges_from_hexagon(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![644325524701193901, 644325524701193754]),
    )]);
}

fn test_h3_get_unidirectional_edge_boundary(file: &mut impl Write) {
    run_ast(file, "h3_get_unidirectional_edge_boundary(0)", &[]);
    run_ast(
        file,
        "h3_get_unidirectional_edge_boundary(1248204388774707199)",
        &[],
    );

    run_ast(file, "h3_get_unidirectional_edge_boundary(h3)", &[(
        "h3",
        UInt64Type::from_data(vec![1248204388774707199, 1319695429381652479]),
    )]);
}

fn test_h3_edge_angle(file: &mut impl Write) {
    run_ast(file, "h3_edge_angle(0)", &[]);
    run_ast(file, "h3_edge_angle(10)", &[]);
    run_ast(file, "h3_edge_angle(16)", &[]);

    run_ast(file, "h3_edge_angle(res)", &[(
        "res",
        UInt8Type::from_data(vec![10, 12]),
    )])
}
