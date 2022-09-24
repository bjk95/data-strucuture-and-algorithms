package linkedlists

import (
	"testing"
)
func TestEmptyListCycle(t *testing.T) {
    list := Constructor()
    if hasCycle(list.Head) {
        t.Fail()
    }
}

func TestHeadLoop(t *testing.T) {
    list := Constructor()
    list.AddAtHead(1)
    list.Head.Next = list.Head
    if !hasCycle(list.Head) {
        t.Fail()
    }
}

func TestCycleStarter(t *testing.T){
    n4 := ListNode{
        Val: 4,
        Next: nil,
    }
    n3 := ListNode{
        Val: 3,
        Next: &n4,
    }
    n2 := ListNode{
        Val: 2,
        Next: &n3,
    }
    n1 := ListNode{
        Val: 1,
        Next: &n2,
    }

    n4.Next = &n2

    cycleStartNode := detectCycle(&n1)
    if cycleStartNode != &n2 {
        t.Fail()
    }
}

func TestRemoveNthFromEnd(t *testing.T) {
    list := Constructor()
    list.AddAtHead(5)
    list.AddAtHead(4)
    list.AddAtHead(3)
    list.AddAtHead(2)
    list.AddAtHead(1)

    newHead := removeNthFromEnd(list.Head, 2)
    if newHead.Next.Next.Val != 3 {
        t.Fail()
    }
}
func TestRemoveNthFromEndSingleElement(t *testing.T) {
    list := Constructor()
    list.AddAtHead(1)

    newHead := removeNthFromEnd(list.Head, 1)
    if newHead != nil {
        t.Fail()
    }
}
func TestRemoveFirstElement(t *testing.T) {
    list := Constructor()
    list.AddAtHead(2)
    list.AddAtHead(1)

    newHead := removeNthFromEnd(list.Head, 2)
    if newHead.Val != 2 {
        t.Fail()
    }
}
func TestRemoveLastElement(t *testing.T) {
    list := Constructor()
    list.AddAtHead(2)
    list.AddAtHead(1)

    newHead := removeNthFromEnd(list.Head, 1)
    if newHead.Val != 1 {
        t.Fail()
    }
}




