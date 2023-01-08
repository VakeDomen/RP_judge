use std::error::Error;
use chrono::Datelike;
use xlsxwriter::{Workbook, FormatUnderline, FormatColor};

use crate::models::student_project::StudentProjectSubmission;

pub fn export_to_xlsx(submissions: Vec<StudentProjectSubmission>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let workbook = Workbook::new(file_path)?;
    let mut sheet =  workbook.add_worksheet(None)?;

    sheet.set_column(0, 0, 25.0, None)?;
    sheet.set_column(1, 5, 10.0, None)?;
    sheet.set_column(6, 7, 17.0, None)?;
    sheet.set_column(8, 14, 27.0, None)?;

    // Write the header row to the sheet
    let headers = ["student_folder", "git_repo", "cloned", "has_task1", "has_task2", "total_commits", "last_commit", "gcc_standard", "commits_task1", "commits_task2", 
    "all_commits_compile_task1", "all_commits_compile_task2", "final_commit_compile_task1", 
    "final_commit_compile_task2", "successful_compiles_task1", "successful_compiles_task2"];
    let header_format = workbook
        .add_format()
        .set_bold();

        let mut col = 0;
    for header in headers.iter() {
        sheet.write_string(0, col, header, Some(&header_format))?;
        col += 1;
    }

    // Iterate through the submissions and write each one to a new row in the sheet
    for (i, submission) in submissions.iter().enumerate() {
        let row = i + 1; // the row index is the submission index + 1 to account for the header row

        // url format
        let url_format = workbook
            .add_format()
            .set_underline(FormatUnderline::Single)
            .set_font_color(FormatColor::Blue);

        // red format
        let red_format = workbook
            .add_format()
            .set_font_color(FormatColor::Red);

        // green format
        let green_format = workbook
            .add_format()
            .set_font_color(FormatColor::Green);

        for header in headers.iter() {
            let column = headers.iter().position(|&r| r == *header).unwrap().try_into().unwrap();

            if *header == "student_folder" {
                sheet.write_string(
                    row.try_into().unwrap(), 
                    column, 
                    &submission.student_folder, 
                    Some(&header_format)
                )?;
            }

            if *header == "git_repo" {
                if let Some(repo) = &submission.git_repo {
                    sheet.write_url(
                        row.try_into().unwrap(), 
                        column,
                        repo, 
                        Some(&url_format)
                    )?;
                }
            }

            if *header == "cloned" {
                let format = if submission.cloned {
                    Some(&green_format)
                } else {
                    Some(&red_format)
                };
                sheet.write_boolean(
                    row.try_into().unwrap(), 
                    column,
                    submission.cloned, 
                    format
                )?;
            }

            if *header == "last_commit" {
                let fmt = if let Some(date) = submission.last_commit_date {
                    if date.day() > 6 && date.month() == 1 {
                        Some(&red_format)
                    } else {
                        Some(&header_format)
                    }
                } else {
                    Some(&header_format)
                };
                if let Some(date) = submission.last_commit_date {
                    sheet.write_string(
                        row.try_into().unwrap(), 
                        column,
                        &date.to_string().clone(), 
                        fmt
                    )?;
                }
            }

            if *header == "gcc_standard" {
                sheet.write_string(
                    row.try_into().unwrap(), 
                    column,
                    &submission.gcc_standard.clone().unwrap_or_else(|| "".to_string()), 
                    Some(&header_format)
                )?;
            }

            if *header == "total_commits" {
                if let Some(val) = submission.total_commits {
                    sheet.write_number(
                        row.try_into().unwrap(), 
                        column,
                        val as f64, 
                        None
                    )?;
                }
            }

            if *header == "commits_task1" {
                if let Some(val) = &submission.commits_task1 {
                    let format = if val.len() > 1 {
                        Some(&green_format)
                    } else {
                        Some(&red_format)
                    };
                    sheet.write_number(
                        row.try_into().unwrap(), 
                        column, 
                        val.len() as f64, 
                        format
                    )?;
                }
            }

            if *header == "commits_task2" {
                if let Some(val) = &submission.commits_task2 {
                    let format = if val.len() > 1 {
                        Some(&green_format)
                    } else {
                        Some(&red_format)
                    };
                    sheet.write_number(
                        row.try_into().unwrap(), 
                        column,
                        val.len() as f64, 
                        format
                    )?;
                }
            }

            if *header == "has_task1" {
                if let Some(val) = submission.has_task1.clone() {
                    sheet.write_string(
                        row.try_into().unwrap(), 
                        headers.iter().position(|&r| r == *header).unwrap().try_into().unwrap(), 
                        val.as_str(), 
                        Some(&header_format)
                    )?;
                } else {
                    sheet.write_boolean(
                        row.try_into().unwrap(), 
                        headers.iter().position(|&r| r == *header).unwrap().try_into().unwrap(), 
                        false, 
                        Some(&red_format)
                    )?;
                }
            }
            
            if *header == "has_task2" {
                if let Some(val) = submission.has_task2.clone() {
                    sheet.write_string(
                        row.try_into().unwrap(), 
                        headers.iter().position(|&r| r == *header).unwrap().try_into().unwrap(), 
                        val.as_str(), 
                        Some(&header_format)
                    )?;
                } else {
                    sheet.write_boolean(
                        row.try_into().unwrap(), 
                        headers.iter().position(|&r| r == *header).unwrap().try_into().unwrap(), 
                        false, 
                        Some(&red_format)
                    )?;
                }
            }

            if *header == "all_commits_compile_task1" {
                if let Some(val) = submission.all_commits_compile_task1 {
                    let format = if val {
                        Some(&green_format)
                    } else {
                        Some(&red_format)
                    };
                    sheet.write_boolean(
                        row.try_into().unwrap(), 
                        headers.iter().position(|&r| r == *header).unwrap().try_into().unwrap(), 
                        val, 
                        format
                    )?;
                }
            }

            if *header == "all_commits_compile_task2" {
                if let Some(val) = submission.all_commits_compile_task2 {
                    let format = if val {
                        Some(&green_format)
                    } else {
                        Some(&red_format)
                    };
                    sheet.write_boolean(
                        row.try_into().unwrap(), 
                        headers.iter().position(|&r| r == *header).unwrap().try_into().unwrap(), 
                        val, 
                        format
                    )?;
                }
            }

            if *header == "final_commit_compile_task1" {
                if let Some(val) = submission.final_commit_compile_task1 {
                    let format = if val {
                        Some(&green_format)
                    } else {
                        Some(&red_format)
                    };
                    sheet.write_boolean(
                        row.try_into().unwrap(), 
                        headers.iter().position(|&r| r == *header).unwrap().try_into().unwrap(), 
                        val, 
                        format
                    )?;
                }
            }

            if *header == "final_commit_compile_task2" {
                if let Some(val) = submission.final_commit_compile_task2 {
                    let format = if val {
                        Some(&green_format)
                    } else {
                        Some(&red_format)
                    };
                    sheet.write_boolean(
                        row.try_into().unwrap(), 
                        headers.iter().position(|&r| r == *header).unwrap().try_into().unwrap(), 
                        val, 
                        format
                    )?;
                }
            }

            if *header == "successful_compiles_task1" {
                if let Some(val) = submission.successful_compiles_task1 {
                    let format = if val > 1 {
                        Some(&green_format)
                    } else {
                        Some(&red_format)
                    };
                    sheet.write_number(
                        row.try_into().unwrap(), 
                        column,
                        val as f64, 
                        format
                    )?;
                }
            }

            if *header == "successful_compiles_task2" {
                if let Some(val) = submission.successful_compiles_task2 {
                    let format = if val > 1 {
                        Some(&green_format)
                    } else {
                        Some(&red_format)
                    };
                    sheet.write_number(
                        row.try_into().unwrap(), 
                        column,
                        val as f64, 
                        format
                    )?;
                }
            }

            col += 1;
        }

        
    }
    workbook.close()?;
    Ok(())
}