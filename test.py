import subprocess
import re

class RgOption:
    def __init__(self, short_flag=None, long_flag=None, argument=None, description=None):
        self.short_flag = short_flag
        self.long_flag = long_flag
        self.argument = argument
        self.description = description

    def __str__(self):
        return f"Short flag: {self.short_flag}, Long flag: {self.long_flag}, Argument: {self.argument}, Description: {self.description}"

def parse_rg_help(output):
    # Split the output into lines for easier processing
    lines = output.split('\n')
    
    # Pattern to match the options and their descriptions
    # This pattern might need adjustments depending on the actual format of rg --help
    option_pattern = re.compile(r'^\s*(-[a-z], )?--([a-zA-Z0-9-]+)( <[^>]+>)?\s+(.*)')
    options = []

    for line in lines:
        match = option_pattern.match(line)
        if match:
            short_flag = match.group(1)[:-2] if match.group(1) else None # Remove comma and space
            long_flag = match.group(2)
            argument = match.group(3) if match.group(3) else None
            description = match.group(4)
            options.append(RgOption(short_flag, long_flag, argument, description))
    
    return options

def get_rg_help():
    # Execute "rg --help" and capture the output
    result = subprocess.run(["rg", "--help"], capture_output=True, text=True)
    if result.returncode == 0:
        return parse_rg_help(result.stdout)
    else:
        print("Error executing 'rg --help':", result.stderr)
        return []

# Example usage
options = get_rg_help()
for option in options:
    print(option)

