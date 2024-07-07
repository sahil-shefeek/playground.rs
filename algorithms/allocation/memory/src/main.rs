use std::io;
use std::process;

struct Process {
    pid: u32,
    mem_requirement: u32,
    is_allocated: bool,
}

struct MemoryBlock {
    block_size: u32,
    allocation: AllocationStatus,
}

enum AllocationStatus {
    Allocated { pid: u32, size: u32 },
    Unallocated,
}

enum SortOrder {
    Ascending,
    Descending,
}

fn sort_memory_blocks(memory_blocks: &mut Vec<MemoryBlock>, sort_order: SortOrder) {
    match sort_order {
        SortOrder::Ascending => {
            for i in 0..(memory_blocks.len() - 1) {
                let mut swap = false;
                for j in 0..(memory_blocks.len() - i - 1) {
                    if memory_blocks[j].block_size > memory_blocks[j + 1].block_size {
                        memory_blocks.swap(j, j + 1);
                        swap = true;
                    }
                }
                if !swap {
                    break;
                };
            }
        }
        SortOrder::Descending => {
            for i in 0..(memory_blocks.len() - 1) {
                let mut swap = false;
                for j in 0..(memory_blocks.len() - i - 1) {
                    if memory_blocks[j].block_size < memory_blocks[j + 1].block_size {
                        memory_blocks.swap(j, j + 1);
                        swap = true;
                    }
                }
                if !swap {
                    break;
                };
            }
        }
    }
}

fn handle_first_fit(processes: &mut Vec<Process>, memory_blocks: &mut Vec<MemoryBlock>) {
    for process in &mut *processes {
        for block in &mut *memory_blocks {
            if process.mem_requirement <= block.block_size {
                match block.allocation {
                    AllocationStatus::Unallocated => {
                        block.allocation = AllocationStatus::Allocated {
                            pid: process.pid,
                            size: process.mem_requirement,
                        };
                        process.is_allocated = true;
                        break;
                    }
                    AllocationStatus::Allocated { pid: _, size: _ } => continue,
                }
            }
        }
    }
    display(processes, memory_blocks);
}

fn handle_best_fit(mut processes: &mut Vec<Process>, mut memory_blocks: &mut Vec<MemoryBlock>) {
    {
        sort_memory_blocks(&mut memory_blocks, SortOrder::Ascending);
    }
    handle_first_fit(&mut processes, &mut memory_blocks);
}
fn handle_worst_fit(mut processes: &mut Vec<Process>, mut memory_blocks: &mut Vec<MemoryBlock>) {
    {
        sort_memory_blocks(&mut memory_blocks, SortOrder::Descending);
    }
    handle_first_fit(&mut processes, &mut memory_blocks);
}

fn get_program_mode_from_user() -> u8 {
    let mut mode = String::new();
    println!("Please enter your choice: ");
    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read input!");
    let mode: u8 = mode.trim().parse().expect("Please enter a valid number!");
    mode
}

fn print_menu() {
    println!(
        "Menu\n1. First-Fit scheduling\n2. Best-Fit scheduling\n3. Worst-Fit scheduling\n4. Exit"
    )
}

fn print_hr(count: usize) {
    for _ in 0..count {
        print!("------");
    }
    println!("");
}

fn display(processes: &Vec<Process>, mem_blocks: &Vec<MemoryBlock>) {
    let mut total_fragmentation: u32 = 0;

    println!("Memory blocks");
    print_hr(processes.len() + mem_blocks.len());
    for block in mem_blocks {
        print!("|  {:>3}  ", block.block_size);
    }
    println!("|");
    print_hr(processes.len() + mem_blocks.len());
    println!("\nMemory blocks after allocation");
    print_hr(processes.len() + mem_blocks.len());
    print!("|");
    for block in mem_blocks {
        match block.allocation {
            AllocationStatus::Allocated { pid: _, size } => {
                let fragmentation = block.block_size - size;
                total_fragmentation += fragmentation;
                print!(" {} + {} |", size, fragmentation)
            }
            AllocationStatus::Unallocated => {
                print!("  {}  |", block.block_size);
            }
        }
    }
    println!("");
    print_hr(processes.len() + mem_blocks.len());
    println!("");
    for block in mem_blocks {
        match block.allocation {
            AllocationStatus::Allocated { pid, size: _ } => {
                print!("   {}   ", pid)
            }
            AllocationStatus::Unallocated => {
                print!("    ");
            }
        }
    }
    println!("");

    for process in processes {
        if !process.is_allocated {
            println!("Process {} not allocated", process.pid)
        }
    }
    println!("");
    println!("Total internal fragmentation = {total_fragmentation}");
}

fn main() {
    let mut num_blocks = String::new();
    println!("Enter the number of memory blocks: ");
    io::stdin()
        .read_line(&mut num_blocks)
        .expect("Failed to read user input");
    let num_blocks: u32 = num_blocks
        .trim()
        .parse()
        .expect("Please enter a valid number.");
    println!("Enter the memory blocks:");
    let mut memory_blocks: Vec<MemoryBlock> = Vec::new();
    for i in 1..=num_blocks {
        let mut mem_size = String::new();
        println!("Enter the size of {i}th memory block:");
        io::stdin()
            .read_line(&mut mem_size)
            .expect("Failed to read user input");
        let mem_size: u32 = mem_size
            .trim()
            .parse()
            .expect("Please enter a valid number.");
        memory_blocks.push(MemoryBlock {
            block_size: mem_size,
            allocation: AllocationStatus::Unallocated,
        });
    }

    let mut num_procs = String::new();
    println!("Enter the number of processes:");
    io::stdin()
        .read_line(&mut num_procs)
        .expect("Failed to read user input");
    let num_procs: u32 = num_procs
        .trim()
        .parse()
        .expect("Please enter a valid number.");
    println!("Enter the processes:");
    let mut processes: Vec<Process> = Vec::new();
    for i in 1..=num_procs {
        let mut mem_requirement = String::new();
        println!("Enter the memory requirement of process {i}:");
        io::stdin()
            .read_line(&mut mem_requirement)
            .expect("Failed to read user input");
        let mem_requirement: u32 = mem_requirement
            .trim()
            .parse()
            .expect("Please enter a valid number.");
        processes.push(Process {
            pid: i,
            mem_requirement,
            is_allocated: false,
        });
    }

    print_menu();
    let mode = get_program_mode_from_user();
    match mode {
        1 => handle_first_fit(&mut processes, &mut memory_blocks),
        2 => handle_best_fit(&mut processes, &mut memory_blocks),
        3 => handle_worst_fit(&mut processes, &mut memory_blocks),
        4 => process::exit(0),
        _ => println!("Please select a valid mode!"),
    }
}
