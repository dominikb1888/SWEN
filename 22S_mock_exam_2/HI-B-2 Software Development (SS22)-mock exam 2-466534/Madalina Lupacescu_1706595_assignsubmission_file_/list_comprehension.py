
def double_list_100(number):
    """ This function doubles a list of numbers based on a maximum number given, if no list is given it doubles all numbers until 1000 """
    double_numbers = []
    numbers = range(number+1)
    for number in numbers:
        double_numbers.append(number * 2)

    #
    double_numbers = [number*2 for number in range(number+1)]
    #
    
    return double_numbers
        


