
def double_list_100(number):
    """ This function doubles each item in a list of at least 100 integers, if no list is given it doubles all numbers until 100 """
    double_numbers = []
    numbers = range(number + 1)
    for number in numbers:
        double_numbers.append(number * 2)

    return double_numbers
        
def double_list_100(number=100):
    return [num*2 for num in range(number+1)]

