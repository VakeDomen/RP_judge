use std::{fs::File, error::Error, io::Write};
use xlsxwriter::{Workbook, FormatUnderline, FormatColor};

use crate::models::student_project::StudentProjectSubmission;

pub fn export_to_csv(submissions: Vec<StudentProjectSubmission>, file_path: &str) -> Result<(), Box<dyn Error>> {
    // Open the file at the given file path in write mode
    let mut file = File::create(file_path)?;

    // Write the CSV header to the file
    let header = "student_folder,git_repo,cloned,commits_task1,commits_task2,has_task1,has_task2,all_commits_compile_task1,all_commits_compile_task2,final_commit_compile_task1,final_commit_compile_task2,successful_compiles_task1,successful_compiles_task2\n";
    file.write_all(header.as_bytes())?;

    // Iterate through the submissions and write each one to a new line in the CSV file
    for submission in submissions {
        let line = format!(
            "{},{},{},{:?},{:?},{},{},{},{},{},{},{},{}\n",
            submission.student_folder,
            submission.git_repo.unwrap_or_else(|| "".to_string()),
            submission.cloned,
            submission.commits_task1.as_ref().map(|v| v.join(" ")).unwrap_or_else(|| "".to_string()),
            submission.commits_task2.as_ref().map(|v| v.join(" ")).unwrap_or_else(|| "".to_string()),
            submission.has_task1.map(|b| b.to_string()).unwrap_or_else(|| "".to_string()),
            submission.has_task2.map(|b| b.to_string()).unwrap_or_else(|| "".to_string()),
            submission.all_commits_compile_task1.map(|b| b.to_string()).unwrap_or_else(|| "".to_string()),
            submission.all_commits_compile_task2.map(|b| b.to_string()).unwrap_or_else(|| "".to_string()),
            submission.final_commit_compile_task1.map(|b| b.to_string()).unwrap_or_else(|| "".to_string()),
            submission.final_commit_compile_task2.map(|b| b.to_string()).unwrap_or_else(|| "".to_string()),
            submission.successful_compiles_task1.map(|b| b.to_string()).unwrap_or_else(|| "".to_string()),
            submission.successful_compiles_task2.map(|b| b.to_string()).unwrap_or_else(|| "".to_string()),
        );
        file.write_all(line.as_bytes())?;
    }

    // Return an empty Ok variant to indicate success
    Ok(())
}

pub fn export_to_xlsx(submissions: Vec<StudentProjectSubmission>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let workbook = Workbook::new(file_path)?;
    let mut sheet =  workbook.add_worksheet(None)?;

    sheet.set_column(0, 0, 25.0, None)?;
    sheet.set_column(1, 4, 10.0, None)?;
    sheet.set_column(5, 6, 17.0, None)?;
    sheet.set_column(7, 13, 27.0, None)?;

    // Write the header row to the sheet
    let headers = ["student_folder", "git_repo", "cloned", "has_task1", "has_task2", "commits_task1", "commits_task2", 
    "all_commits_compile_task1", "all_commits_compile_task2", "final_commit_compile_task1", 
    "final_commit_compile_task2", "successful_compiles_task1", "successful_compiles_task2"];
    let header_format = workbook
        .add_format()
        .set_bold();
    let mut col = 0;
    for header in headers.iter() {
        sheet.write_string(0, col, &header, Some(&header_format))?;
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
                sheet.write_url(
                    row.try_into().unwrap(), 
                    column,
                    &submission.git_repo.clone().unwrap_or_else(|| "".to_string()), 
                    Some(&url_format)
                )?;
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
                if let Some(val) = submission.has_task1 {
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

            if *header == "has_task2" {
                if let Some(val) = submission.has_task2 {
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