package linkedlists


func hasCycle(head *ListNode) bool {
	listHasCycle := false
    if head != nil && head.Next != nil{
		listHasCycle = fastEqualSlow(head.Next, head)
	}
	return listHasCycle
}

func fastEqualSlow(fast *ListNode, slow *ListNode) bool {
	if fast == nil || fast.Next == nil || slow.Next == nil {
		return false
	} else if fast == slow {
		return true
	} 
	return fastEqualSlow(fast.Next.Next, slow.Next)
}
func detectCycle(head *ListNode) *ListNode {
	var intersectionNode *ListNode
    if head != nil && head.Next != nil{
		intersectionNode = fastEqualSlowIntersectionNode(head.Next, head, head)
	}
	return findCycleRootNode(head, intersectionNode)
	
}

func fastEqualSlowIntersectionNode(fast *ListNode, slow *ListNode, last *ListNode) *ListNode {
	if fast == nil || fast.Next == nil || slow.Next == nil {
		return nil
	} else if fast == slow {
		return last
	} 
	return fastEqualSlowIntersectionNode(fast.Next.Next, slow.Next, slow)
}

func findCycleRootNode(n1 *ListNode, n2 *ListNode) *ListNode {
	if n1.Next == n2.Next {
		return n1
	} else {
		return findCycleRootNode(n1.Next, n2.Next)
	}
}

func getIntersectionNode(headA, headB *ListNode) *ListNode {
    p1 := headA
	p2 := headB

	for p1 != p2 {
		if p2.Next != nil {
			p2 = p2.Next
		} else if p1.Next != nil {
			p1 = p1.Next
			p2 = headB
		} else {
			return nil
		}		
	}

	return p1
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	dummy := ListNode{
		Val: 0,
		Next: head,
	}
    leadPointer := &dummy
	trailingPointer := &dummy


	for i := 0; i < n; i++ {
		leadPointer = leadPointer.Next
	}

	for leadPointer.Next != nil {
		leadPointer = leadPointer.Next
		trailingPointer = trailingPointer.Next
	}

	trailingPointer.Next = trailingPointer.Next.Next
	return dummy.Next
}