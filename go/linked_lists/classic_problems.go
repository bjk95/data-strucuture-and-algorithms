package linkedlists

func reverseList(head *ListNode) *ListNode {
	var previous *ListNode 
	current := head
	for current != nil {
		tmp := current.Next // 2 -> 3 -> 4
		current.Next = previous // nil -> 1 -> 2
		previous = current // 1 -> 2 -> 
		current = tmp // 2 -> 3
	}

	return previous
}

func removeElements(head *ListNode, val int) *ListNode {
	sentinel := &ListNode{
		Next: head,
	}
	previous := sentinel
	current := head
	for current != nil {
		if current.Val == val {
			previous.Next = current.Next
		} else {
			previous = current
		}
		current = current.Next
	}
	return sentinel.Next 
}

func oddEvenList(head *ListNode) *ListNode {
	odd := head
	even := head.Next
	evenHead := even

	for even != nil && even.Next != nil {
		odd.Next = even.Next
		odd = odd.Next
		even.Next = odd.Next
		even = odd
	}

	odd.Next = evenHead

    return head
}

func isPalindrome(head *ListNode) bool {
	current := head
	array := make([]int, 0)
	for current != nil {
		array = append(array, current.Val)
		current = current.Next
	}

	isPalindrome := true
	start := 0
	end := len(array) - 1
	for start < end {
		if array[start] != array[end]{
			isPalindrome = false
			break
		} else {
			start++
			end--
		}
	}
	return isPalindrome  
}