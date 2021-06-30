Import-Module "$PSScriptRoot/../siquery"

Describe 'groups table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll { $allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json" }
			It 'has a schema' {
				$groups = $allSchemas | Where-Object { $_.BaseName -eq 'groups' }
				$groups | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$groups = $allSchemas | Where-Object { $_.BaseName -eq 'groups' }
				$schema = Get-Content $groups.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			It 'validates schema' {
				$groups = Get-Siq 'groups' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'groups' }).FullName -Raw

				$json = $groups | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
