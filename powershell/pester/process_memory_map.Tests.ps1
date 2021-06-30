Import-Module "$PSScriptRoot/../siquery"

Describe 'process_memory_map table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll { $allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json" }
			It 'has a schema' {
				$process_memory_map = $allSchemas | Where-Object { $_.BaseName -eq 'process_memory_map' }
				$process_memory_map | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$process_memory_map = $allSchemas | Where-Object { $_.BaseName -eq 'process_memory_map' }
				$schema = Get-Content $process_memory_map.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			It 'validates schema' {
				$process_memory_map = Get-Siq 'process_memory_map' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'process_memory_map' }).FullName -Raw

				$json = $process_memory_map | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
