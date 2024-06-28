def johny_progress_days(miles: list) -> int:
    """
    Count the number of days Johny made progress.

    This function takes a list of miles Johny ran each day and counts the number of days
    he ran more miles than the previous day.

    Parameters:
    miles (list): A list of integers representing the miles Johny ran each day.

    Returns:
    int: The number of days Johny made progress by running more miles than the previous day.
    """
    progress_days = 0
    for i in range(len(miles) - 1):
        if miles[i+1] > miles[i]:
            progress_days += 1
    return progress_days
