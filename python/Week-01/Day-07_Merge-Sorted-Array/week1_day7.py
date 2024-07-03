def merge_sorted_array(nums1, nums2):
    m = len(nums1) - len(nums2)
    n = len(nums2)
    nums1[m:] = nums2
    return sorted(nums1) if n > 0 else nums1


# Alternative algo
# def merge_sorted_array(nums1: list, nums2: list) -> list:
#     m = len(nums1) - len(nums2)
#     n = len(nums2)
#
#     i, j, k = m - 1, n - 1, m + n - 1
#
#     while i >= 0 and j >= 0:
#         if nums1[i] > nums2[j]:
#             nums1[k] = nums1[i]
#             i -= 1
#         else:
#             nums1[k] = nums2[j]
#             j -= 1
#         k -= 1
#
#     while j >= 0:
#         nums1[k] = nums2[j]
#         j -= 1
#         k -= 1
#
#     return nums1
