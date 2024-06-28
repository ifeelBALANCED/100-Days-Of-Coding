def johny_progress_days(miles: list) -> int:
    progress_days = 0
    for i in range(len(miles) - 1):
        if miles[i+1] > miles[i]:
            progress_days += 1
    return progress_days
