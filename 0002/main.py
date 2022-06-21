# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        digit = l1.val + l2.val
        first = ListNode(val=digit % 10)
        remaining = digit // 10
        prev = first
        l1 = l1.next
        l2 = l2.next
        curr = None

        while l1 != None and l2 != None:
            digit = l1.val + l2.val + remaining
            curr = ListNode(val=digit % 10)
            remaining = digit // 10
            prev.next = curr
            l1 = l1.next
            l2 = l2.next
            prev = curr
        
        active = l1 if l1 != None else l2 if l2 != None else None
        while active != None:
            curr = ListNode(val=(active.val+remaining) % 10)
            remaining = (active.val + remaining) // 10
            prev.next = curr
            active = active.next
            prev = curr

        if remaining != 0:
            prev.next = ListNode(val=remaining)
        
        return first

if __name__ == "__main__":
    l1 = ListNode(val=3)
    l1 = ListNode(val=4, next=l1)
    l1 = ListNode(val=2, next=l1)

    l2 = ListNode(val=4)
    l2 = ListNode(val=6, next=l2)
    l2 = ListNode(val=5, next=l2)

    res = Solution().addTwoNumbers(l1, l2)
    assert(res.val == 7)
    assert(res.next.val == 0)
    assert(res.next.next.val == 8)
